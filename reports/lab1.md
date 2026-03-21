# Lab1 报告（ch3）

## 功能总结（200字以内）

本次实现了 `sys_trace`（ID=410）三个功能：`trace_request=0` 按字节读取当前任务地址空间数据，`trace_request=1` 按字节写入数据，`trace_request=2` 查询当前任务某系统调用的调用次数（包含本次 `sys_trace` 调用）。在内核 `syscall` 入口统一计数，并在任务管理器中为每个任务维护独立的 syscall 计数表。为适配本地 RustSBI/QEMU 运行环境，还修复了 `get_time` 在早期返回 0 导致 ch3_sleep 失败的问题，并将 `shutdown` 调整为 `wfi` 等待，避免反复 panic 干扰输出。

## 简答题

### 1. U 态下执行 S 态相关非法操作的行为

测试环境：`RustSBI-QEMU Version 0.2.0-alpha.2`（启动信息可见），SBI 协议显示 `RustSBI 0.3.1 adapting to SBI v1.0.0`。

- `ch2b_bad_address.rs`：向 `0x0` 写，触发 `StoreFault/PageFault`，内核打印 page fault 并杀死应用。
- `ch2b_bad_instructions.rs`：执行 `sret`，触发非法指令，内核打印 `IllegalInstruction` 并杀死应用。
- `ch2b_bad_register.rs`：读取 `sstatus`（特权 CSR），触发非法指令，内核打印 `IllegalInstruction` 并杀死应用。

### 2. trap.S 中 `__alltraps` 与 `__restore`

1) 刚进入 `__restore` 时 `sp` 是**内核栈上 TrapContext 基址**。两种场景：
- 内核首次调度任务，通过 `TaskContext::goto_restore(init_app_cx(...))` 跳转进 `__restore`；
- Trap 处理完从 `trap_handler` 返回后继续执行 `__restore`，恢复用户态。

2) L43-L48 特殊处理 `sstatus/sepc/sscratch`：
- `sstatus`：包含 SPP/SPIE 等返回特权与中断状态；
- `sepc`：`sret` 返回的 PC（用户下一条指令地址）；
- `sscratch`：保存用户栈顶（与 `sp` 交换时使用）。

3) 跳过 `x2(sp)` 与 `x4(tp)`：
- `x2` 由后续 `csrrw sp, sscratch, sp` 专门恢复；
- `x4`（tp）在该实验用户程序不依赖，不必恢复。

4) L60 后：
- `sp` 变为用户栈指针；
- `sscratch` 变为内核栈指针（供下次 Trap 入口交换）。

5) `__restore` 中状态切换发生在 `sret`。因为 `sret` 会依据 `sstatus.SPP` 恢复到 U 态并跳转到 `sepc`。

6) L13（`__alltraps` 开头）后：
- `sp` 变为内核栈指针；
- `sscratch` 暂存原用户栈指针。

7) 从 U 态进入 S 态由用户执行 `ecall` 指令触发（也可由异常/中断触发陷入）。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

> *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

> *rCore-Tutorial-Guide 第三章练习页面、课程提供的代码与测试仓库*

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
