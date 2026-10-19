use std::collections::HashMap;

use crate::analysis::Function;
use crate::analysis::FunctionAnalysis;
use crate::analysis::LivePath;
use crate::analysis::Refusal;
use crate::audit;
use crate::audit::ScopeKind;
use crate::isa::Gpr;
use crate::isa::GprSet;
use crate::labels::LabelReferences;
use crate::line::Line;
use crate::rewrite::Listings;
use crate::rewrite::Report;
use crate::rewrite::rewrite;

/// Wraps `body` in the framing LLVM prints around a compiled function.
fn compiled(name: &str, body: &str) -> String {
    format!(
        "\t.section\t.text.{name},\"ax\",@progbits\n\t.p2align\t2\n\t.type\t{name},@function\n\
         {name}:\n.Lfunc_begin0:\n{body}.Lfunc_end0:\n\t.size\t{name}, .Lfunc_end0-.Lfunc_begin0\n"
    )
}

fn rewritten(source: &str) -> (String, Report) {
    let mut output = Vec::new();
    let listings = Listings {
        scope: Vec::new(),
        refusals: Vec::new(),
    };
    let references = LabelReferences::scan(source.as_bytes()).expect("scan");
    let report = rewrite(source.as_bytes(), &references, &mut output, listings).expect("rewrite");
    (String::from_utf8(output).expect("utf-8"), report)
}

fn analyze(body: &str) -> FunctionAnalysis {
    Function::new(body.lines().map(Line::parse).collect(), |_| true).analyze()
}

fn prologue_refusal(body: &str) -> Refusal {
    match analyze(body).prologue {
        Some(Err(refusal)) => refusal,
        other => panic!("expected a refused prologue, got {other:?}"),
    }
}

fn epilogue_refusal(body: &str) -> Refusal {
    let analysis = analyze(body);
    match analysis.epilogues.as_slice() {
        [Err(refusal)] => refusal.clone(),
        other => panic!("expected one refused epilogue, got {other:?}"),
    }
}

#[test]
fn rewrites_interleaved_prologue_and_epilogue() {
    let source = compiled(
        "socket_error",
        "\tmflr 0
\tstwu 1, -32(1)
\tstw 0, 36(1)
\tstw 28, 16(1)
\tmr\t28, 3
\tli 3, 3
\tstw 29, 20(1)
\tmr\t29, 4
\tstw 30, 24(1)
\tmr\t30, 5
\tbl NetDll_WSAGetLastError
\tcmplwi\t3, 10035
\tbeq\t0, .LBB0_2
\tmr\t3, 28
\tbl disconnect
.LBB0_2:
\tlwz 30, 24(1)
\tlwz 29, 20(1)
\tlwz 28, 16(1)
\tlwz 0, 36(1)
\taddi 1, 1, 32
\tmtlr 0
\tblr
",
    );
    let (output, report) = rewritten(&source);
    insta::assert_snapshot!(output);
    assert_eq!(report.prologues.rewritten, 1);
    assert_eq!(report.epilogues.rewritten, 1);
    // 3 stores, the link store, 3 loads, `lwz 0`, `mtlr`, `addi`, `blr`.
    assert_eq!(report.removed_instructions, 11);
    assert_eq!(report.added_instructions, 3);
}

#[test]
fn moves_link_read_after_allocation() {
    let source = compiled(
        "late_mflr",
        "\tstwu 1, -16(1)
\tmflr 0
\tstw 0, 20(1)
\tstw 30, 8(1)
\tmr\t30, 3
\tbl callee
\tmr\t3, 30
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
",
    );
    insta::assert_snapshot!(rewritten(&source).0);
}

#[test]
fn uses_chain_ending_at_r31() {
    let source = compiled(
        "frame_pointer",
        "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tstw 31, 12(1)
\tmr\t31, 1
\tbl callee
\tlwz 31, 12(1)
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
",
    );
    insta::assert_snapshot!(rewritten(&source).0);
}

#[test]
fn rewrites_link_only_and_multiple_epilogues() {
    let source = compiled(
        "two_returns",
        "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tbl callee
\tcmplwi\t3, 0
\tbeq\t0, .LBB0_2
\tli 3, 1
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
.LBB0_2:
\tlwz 0, 20(1)
\tli 3, 2
\taddi 1, 1, 16
\tmtlr 0
\tblr
",
    );
    let (output, report) = rewritten(&source);
    insta::assert_snapshot!(output);
    assert_eq!(report.prologues.rewritten, 0);
    assert_eq!(report.epilogues.rewritten, 2);
}

/// LLVM's long conditional branch in a function larger than the range of
/// `bc`: an inverted branch over `b`. The target of `.+8` is the first
/// restore of an epilogue, which the rewrite removes, so the branch must
/// name a label rather than a byte offset.
#[test]
fn rewrites_function_with_relaxed_branches() {
    let source = compiled(
        "relaxed_branches",
        "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tmr\t30, 3
\tbl callee
\tcmplwi\t3, 0
\tbc 12, 2, .+8
\tb .LBB0_2
.Ltmp0:
\t.loc\t1 7 3
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
.LBB0_2:
\tmr\t3, 30
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
",
    );
    let (output, report) = rewritten(&source);
    insta::assert_snapshot!(output);
    assert_eq!(report.prologues.rewritten, 1);
    assert_eq!(report.epilogues.rewritten, 2);
    assert!(report.prologues.refused.is_empty());
    assert!(report.epilogues.refused.is_empty());
}

#[test]
fn leaves_module_assembly_unchanged() {
    // Naked functions and global assembly carry no `.Lfunc_begin` label.
    let source = "\t.section\t.text.CreateEventA,\"ax\",@progbits
\t.p2align\t2
\t.globl\tCreateEventA
\t.type\tCreateEventA,@function
CreateEventA:
\tmflr\t0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl .Ltmp0
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\tmtlr\t0
\taddi 1, 1, 16
\tblr
.Lfunc_end_CreateEventA:
.Ltmp1:
\t.size\tCreateEventA, .Ltmp1-CreateEventA
";
    let (output, report) = rewritten(source);
    assert_eq!(output, source);
    assert_eq!(report.functions, 0);
}

#[test]
fn leaves_frameless_functions_unchanged() {
    let source = compiled(
        "leaf",
        "\tlwz 3, 0(3)
\taddi 3, 3, 1
\tblr
",
    );
    let (output, report) = rewritten(&source);
    assert_eq!(output, source);
    assert_eq!(report.functions, 1);
    assert_eq!(report.prologues.rewritten + report.epilogues.rewritten, 0);
}

#[test]
fn refuses_without_link_register_save() {
    let body = "\tstwu 1, -16(1)
\tstw 30, 8(1)
\tmr\t30, 3
\taddi 3, 30, 1
\tlwz 30, 8(1)
\taddi 1, 1, 16
\tblr
";
    assert_eq!(prologue_refusal(body), Refusal::LinkRegisterNotSaved);
    assert_eq!(epilogue_refusal(body), Refusal::LinkRegisterNotRestored);
}

#[test]
fn refuses_non_contiguous_ranges() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 28, 0(1)
\tstw 30, 8(1)
\tbl callee
\tlwz 30, 8(1)
\tlwz 28, 0(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
";
    let registers: GprSet = [28, 30].into_iter().filter_map(Gpr::new).collect();
    assert_eq!(prologue_refusal(body), Refusal::NonContiguous { registers });
    assert_eq!(epilogue_refusal(body), Refusal::NonContiguous { registers });
}

#[test]
fn refuses_register_written_before_its_save() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tmr\t30, 3
\tstw 30, 8(1)
\tbl callee
";
    let register = Gpr::new(30).expect("gpr");
    assert_eq!(
        prologue_refusal(body),
        Refusal::WrittenBeforeSave { register }
    );
}

#[test]
fn refuses_r0_written_before_link_store() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 30, 8(1)
\tli 0, 5
\tstw 0, 20(1)
\tbl callee
";
    assert_eq!(
        prologue_refusal(body),
        Refusal::WrittenBeforeSave { register: Gpr::R0 }
    );
}

#[test]
fn refuses_slot_access_before_save() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tlwz 3, 8(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
";
    assert_eq!(prologue_refusal(body), Refusal::SlotAccessed);
}

#[test]
fn refuses_stack_pointer_write_before_save() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tmr\t1, 3
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
";
    assert_eq!(prologue_refusal(body), Refusal::StackPointerAccessed);
}

#[test]
fn refuses_unknown_instruction_in_window() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstwcx. 3, 0, 4
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
";
    assert_eq!(
        prologue_refusal(body),
        Refusal::UnknownInstruction {
            mnemonic: "stwcx.".to_owned()
        }
    );
}

#[test]
fn refuses_slot_overlapping_back_chain() {
    let body = "\tmflr 0
\tstwu 1, -8(1)
\tstw 0, 12(1)
\tstw 30, 0(1)
\tbl callee
";
    assert_eq!(prologue_refusal(body), Refusal::SlotOverlapsBackChain);
}

#[test]
fn refuses_multiple_allocations() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
\tstwu 1, -16(1)
";
    assert_eq!(prologue_refusal(body), Refusal::MultipleAllocations);
}

#[test]
fn refuses_link_register_read_after_save() {
    let read = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tmflr 3
\tbl callee
";
    assert_eq!(
        prologue_refusal(read),
        Refusal::LinkRegisterLive {
            path: LivePath::Read {
                mnemonic: "mflr".to_owned()
            }
        }
    );
    let conditional_return = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tcmplwi\t3, 0
\tbeq\t0, .LBB0_2
\tbl callee
.LBB0_2:
\tbeqlr\t0
\tbl callee
";
    assert_eq!(
        prologue_refusal(conditional_return),
        Refusal::LinkRegisterLive {
            path: LivePath::Read {
                mnemonic: "beqlr".to_owned()
            }
        }
    );
    let tail_branch = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbne\t0, .LBB0_1
\tb elsewhere
.LBB0_1:
\tbl callee
";
    assert_eq!(
        prologue_refusal(tail_branch),
        Refusal::LinkRegisterLive {
            path: LivePath::TailBranch {
                target: "elsewhere".to_owned()
            }
        }
    );
    let inline_asm = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\t#APP
\tmfspr 3, 8
\t#NO_APP
\tbl callee
";
    assert_eq!(
        prologue_refusal(inline_asm),
        Refusal::LinkRegisterLive {
            path: LivePath::InlineAsm
        }
    );
    let falls_off_end = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tmr\t30, 3
";
    assert_eq!(
        prologue_refusal(falls_off_end),
        Refusal::LinkRegisterLive {
            path: LivePath::FallsOffEnd
        }
    );
}

#[test]
fn accepts_loops_traps_and_acquire_branches_after_save() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
.LBB0_1:
\tlwz 5, 0(4)
\tcmpw 7, 5, 5
\tbne- 7, .+4
\tisync
\taddi 3, 3, -1
\tcmplwi\t3, 0
\tbne\t0, .LBB0_1
\tcmplwi\t4, 0
\tbne\t0, .LBB0_3
\ttrap
.LBB0_3:
\tbl callee
";
    assert!(matches!(analyze(body).prologue, Some(Ok(_))));
}

#[test]
fn refuses_register_read_after_restore() {
    let body = "\tlwz 30, 8(1)
\tmr\t3, 30
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
";
    let register = Gpr::new(30).expect("gpr");
    assert_eq!(
        epilogue_refusal(body),
        Refusal::AccessedAfterRestore { register }
    );
}

#[test]
fn refuses_store_to_slot_after_restore() {
    let body = "\tlwz 30, 8(1)
\tstw 3, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
";
    assert_eq!(epilogue_refusal(body), Refusal::SlotAccessed);
}

#[test]
fn refuses_stack_pointer_use_after_deallocation() {
    let body = "\tlwz 0, 20(1)
\taddi 1, 1, 16
\tstw 3, -4(1)
\tmtlr 0
\tblr
";
    assert_eq!(epilogue_refusal(body), Refusal::StackPointerAccessed);
}

#[test]
fn refuses_link_register_use_after_restore() {
    let body = "\tlwz 0, 20(1)
\tmtlr 0
\tmflr 3
\taddi 1, 1, 16
\tblr
";
    assert_eq!(epilogue_refusal(body), Refusal::LinkRegisterAccessed);
}

#[test]
fn refuses_unknown_instruction_in_epilogue() {
    let body = "\tlwz 30, 8(1)
\tsync
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
";
    assert_eq!(
        epilogue_refusal(body),
        Refusal::UnknownInstruction {
            mnemonic: "sync".to_owned()
        }
    );
}

#[test]
fn ignores_returns_inside_inline_asm() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tbl callee
\t#APP
\tlwz 0, 20(1)
\tmtlr 0
\taddi 1, 1, 16
\tblr
\t#NO_APP
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
";
    let analysis = analyze(body);
    assert!(analysis.inline_asm);
    assert_eq!(analysis.epilogues.len(), 1);
    assert_eq!(
        analysis.epilogues[0].as_ref().map(|rewrite| rewrite.ret),
        Ok(13)
    );
}

#[test]
fn refuses_functions_with_relative_branches() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
\t#APP
\tbne 0, .+16
\t#NO_APP
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\tmtlr 0
\taddi 1, 1, 16
\tblr
";
    let refusal = Refusal::RelativeBranch {
        target: ".+16".to_owned(),
    };
    assert_eq!(prologue_refusal(body), refusal);
    assert_eq!(epilogue_refusal(body), refusal);
    let statements = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
\t#APP
\tcmpwi 3, 0; beq .+16
\t#NO_APP
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\tmtlr 0
\taddi 1, 1, 16
\tblr
";
    assert_eq!(prologue_refusal(statements), Refusal::MultipleStatements);
    assert_eq!(epilogue_refusal(statements), Refusal::MultipleStatements);
}

#[test]
fn refuses_epilogue_overlapping_prologue() {
    let body = "\tmflr 0
\tstwu 1, -16(1)
\tstw 30, 8(1)
\tlwz 30, 8(1)
\tstw 0, 20(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
";
    let analysis = analyze(body);
    assert!(matches!(analysis.prologue, Some(Ok(_))));
    assert_eq!(analysis.epilogues, vec![Err(Refusal::OverlapsPrologue)]);
}

/// Function as LLVM prints it with `-Cdebuginfo`: line annotations and
/// local labels inside the prologue and epilogue, call frame annotations,
/// and debug sections that name the labels.
#[test]
fn rewrites_function_with_debug_information() {
    let source = format!(
        "\t.cfi_sections .debug_frame\n{}\t.section\t.debug_info,\"\",@progbits\n\t.long\t.Ltmp0\n\t.long\t.Ltmp1\n",
        compiled(
            "with_debug_information",
            "\t.loc\t1 63 0
\t.cfi_startproc
\tmflr 0
\tstwu 1, -32(1)
\tstw 0, 36(1)
\t.cfi_def_cfa_offset 32
\t.cfi_offset lr, 4
\t.cfi_offset r29, -12
\t.cfi_offset r30, -8
\t.cfi_offset cr2, -16
\tmfcr 12
.Ltmp0:
\t.loc\t2 10 9 prologue_end
\tstw 29, 20(1)
\tmr\t29, 3
\t.loc\t2 0 9 is_stmt 0
\tstw 30, 24(1)
\tstw 12, 16(1)
\tbl callee
.Ltmp1:
\t.loc\t2 12 1 epilogue_begin
\tlwz 12, 16(1)
\tmtcrf 32, 12
\tlwz 30, 24(1)
\tlwz 29, 20(1)
\tlwz 0, 36(1)
\taddi 1, 1, 32
\tmtlr 0
\tblr
\t.cfi_endproc
",
        )
    );
    let (output, report) = rewritten(&source);
    insta::assert_snapshot!(output);
    assert_eq!(report.prologues.rewritten, 1);
    assert_eq!(report.epilogues.rewritten, 1);
}

/// A jump table in a loaded section names `.Ltmp0`, so control may enter
/// there and the saves after it are outside the allocation block.
#[test]
fn keeps_block_boundary_at_referenced_local_label() {
    let source = format!(
        "{}\t.section\t.rodata,\"a\",@progbits\n\t.long\t.Ltmp0\n",
        compiled(
            "entered_label",
            "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
.Ltmp0:
\tstw 30, 8(1)
\tbl callee
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\tmtlr 0
\tblr
",
        )
    );
    let (_, report) = rewritten(&source);
    assert_eq!(report.prologues.rewritten, 0);
}

#[test]
fn refuses_unwind_annotation_between_moved_instructions() {
    let epilogue = "\tmflr 0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
\tlwz 30, 8(1)
\tlwz 0, 20(1)
\taddi 1, 1, 16
\t.cfi_def_cfa_offset 0
\tmtlr 0
\tblr
";
    assert_eq!(
        epilogue_refusal(epilogue),
        Refusal::UnwindAnnotation {
            annotation: ".cfi_def_cfa_offset 0".to_owned()
        }
    );

    let prologue = "\tmflr 0
\t.cfi_register lr, r0
\tstwu 1, -16(1)
\tstw 0, 20(1)
\tstw 30, 8(1)
\tbl callee
";
    assert_eq!(
        prologue_refusal(prologue),
        Refusal::UnwindAnnotation {
            annotation: ".cfi_register lr, r0".to_owned()
        }
    );
}

#[test]
fn audit_reports_remaining_sequences() {
    let disassembly = "
/tmp/image:\tfile format elf32-powerpc

Disassembly of section .text:

91f01000 <rewritten>:
91f01000:      \tmflr 0
91f01004:      \tbl 0x91f01040 <__xbox360_savegprlr_30_30>
91f01008:      \tstwu 1, -16(1)
91f0100c:      \tbl 0x91f02000 <callee>
91f01010:      \taddi 1, 1, 16
91f01014:      \tb 0x91f01048 <__xbox360_restgprlr_30_30>

91f01018 <remaining>:
91f01018:      \tmflr 0
91f0101c:      \tstwu 1, -16(1)
91f01020:      \tstw 0, 20(1)
91f01024:      \tstw 30, 8(1)
91f01028:      \tbf\t2, 0x91f01030 <remaining+0x18>
91f0102c:      \tbl 0x91f02000 <callee>
91f01030:      \tlwz 30, 8(1)
91f01034:      \tlwz 0, 20(1)
91f01038:      \taddi 1, 1, 16
91f0103c:      \tmtlr 0
91f01040:      \tblr

91f01044 <__xbox360_savegprlr_30_30>:
91f01044:      \tstw 30, -8(1)
";
    let scope: HashMap<String, ScopeKind> = [
        ("rewritten".to_owned(), ScopeKind::Plain),
        ("remaining".to_owned(), ScopeKind::Plain),
    ]
    .into_iter()
    .collect();
    let report = audit::audit(disassembly.as_bytes(), &scope).expect("audit");
    assert_eq!(report.functions_checked, 2);
    assert_eq!(report.helper_branches, 2);
    assert_eq!(report.prologues.rewritten, 1);
    assert_eq!(report.epilogues.rewritten, 1);
    assert!(!report.passed());
    assert_eq!(
        report
            .rewritable
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["remaining"]
    );
}
