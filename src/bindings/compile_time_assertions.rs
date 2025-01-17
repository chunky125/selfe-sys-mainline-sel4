/// Compile-time checks to confirm that the generated API meets our human-identified expectations.
/// These checks help identify if the underlying seL4 build or bindings generation step have
/// gone awry.
use crate::bindings::*;

#[allow(dead_code)]
#[doc(hidden)]
fn existence_and_size_checks() {
    // Use transmute to tell the compiler to confirm type size equality at compile time
    // Note that we could be using the `static_assertions` lib instead, but are choosing
    // to trim dependencies aggressively for now.
    use core::mem::transmute as assert_eq_size;
    let _ = assert_eq_size::<seL4_Word, seL4_CNode_CapData>;
    let _ = assert_eq_size::<seL4_Word, seL4_CapRights>;
    let _ = assert_eq_size::<seL4_Word, seL4_MessageInfo>;
    let _ = assert_eq_size::<seL4_CPtr, seL4_CNode>;
    let _ = assert_eq_size::<seL4_CPtr, seL4_IRQHandler>;
    let _ = assert_eq_size::<seL4_CPtr, seL4_IRQControl>;
    let _ = assert_eq_size::<seL4_CPtr, seL4_TCB>;
    let _ = assert_eq_size::<seL4_CPtr, seL4_Untyped>;
    let _ = assert_eq_size::<seL4_CPtr, seL4_DomainSet>;

    let _: seL4_UserContext;
    let _: seL4_Fault;
    let _: seL4_IPCBuffer;
    let _: seL4_Error;
    let _: seL4_Bool;
    let _: seL4_BootInfo;
}

// Core common functions that are not syscalls
const SEL4_GETMR: unsafe extern "C" fn(ctypes::c_int) -> seL4_Word = seL4_GetMR;
const SEL4_SETMR: unsafe extern "C" fn(ctypes::c_int, seL4_Word) = seL4_SetMR;
const SEL4_GETUSERDATA: unsafe extern "C" fn() -> seL4_Word = seL4_GetUserData;
const SEL4_SETUSERDATA: unsafe extern "C" fn(seL4_Word) = seL4_SetUserData;
const SEL4_GETBADGE: unsafe extern "C" fn(ctypes::c_int) -> seL4_Word = seL4_GetBadge;
const SEL4_GETCAP: unsafe extern "C" fn(ctypes::c_int) -> seL4_CPtr = seL4_GetCap;
const SEL4_SETCAP: unsafe extern "C" fn(ctypes::c_int, seL4_CPtr) = seL4_SetCap;
const SEL4_GETIPCBUFFER: unsafe extern "C" fn() -> *mut seL4_IPCBuffer = seL4_GetIPCBuffer;

// Syscalls
const SEL4_SEND: unsafe extern "C" fn(seL4_CPtr, seL4_MessageInfo) = seL4_Send;
const SEL4_NBSEND: unsafe extern "C" fn(seL4_CPtr, seL4_MessageInfo) = seL4_NBSend;
const SEL4_REPLY: unsafe extern "C" fn(seL4_MessageInfo) = seL4_Reply;
const SEL4_SIGNAL: unsafe extern "C" fn(seL4_CPtr) = seL4_Signal;
const SEL4_RECV: unsafe extern "C" fn(seL4_CPtr, *mut seL4_Word) -> seL4_MessageInfo = seL4_Recv;
const SEL4_NBRECV: unsafe extern "C" fn(seL4_CPtr, *mut seL4_Word) -> seL4_MessageInfo =
    seL4_NBRecv;
const SEL4_CALL: unsafe extern "C" fn(seL4_CPtr, seL4_MessageInfo) -> seL4_MessageInfo = seL4_Call;
const SEL4_REPLYRECV: unsafe extern "C" fn(
    seL4_CPtr,
    seL4_MessageInfo,
    *mut seL4_Word,
) -> seL4_MessageInfo = seL4_ReplyRecv;
const SEL4_YIELD: unsafe extern "C" fn() = seL4_Yield;
const SEL4_WAIT: unsafe extern "C" fn(seL4_CPtr, *mut seL4_Word) = seL4_Wait;
const SEL4_POLL: unsafe extern "C" fn(seL4_CPtr, *mut seL4_Word) -> seL4_MessageInfo = seL4_Poll;

// Target-independent API functions
const UNTYPED_RETYPE: unsafe extern "C" fn(
    seL4_Untyped,
    seL4_Word,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_Word,
    seL4_Word,
    seL4_Word,
) -> seL4_Error = seL4_Untyped_Retype;
const TCB_READREGISTERS: unsafe extern "C" fn(
    seL4_TCB,
    seL4_Bool,
    seL4_Uint8,
    seL4_Word,
    *mut seL4_UserContext,
) -> seL4_Error = seL4_TCB_ReadRegisters;
const TCB_WRITEREGISTERS: unsafe extern "C" fn(
    seL4_TCB,
    seL4_Bool,
    seL4_Uint8,
    seL4_Word,
    *mut seL4_UserContext,
) -> seL4_Error = seL4_TCB_WriteRegisters;
const TCB_COPYREGISTERS: unsafe extern "C" fn(
    seL4_TCB,
    seL4_TCB,
    seL4_Bool,
    seL4_Bool,
    seL4_Bool,
    seL4_Bool,
    seL4_Uint8,
) -> seL4_Error = seL4_TCB_CopyRegisters;
const TCB_CONFIGURE: unsafe extern "C" fn(
    seL4_TCB,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_Word,
    seL4_CPtr,
) -> seL4_Error = seL4_TCB_Configure;
const TCB_SETPRIORITY: unsafe extern "C" fn(seL4_TCB, seL4_CPtr, seL4_Word) -> seL4_Error =
    seL4_TCB_SetPriority;
const TCB_SETMCPRIORITY: unsafe extern "C" fn(seL4_TCB, seL4_CPtr, seL4_Word) -> seL4_Error =
    seL4_TCB_SetMCPriority;
const TCB_SETSCHEDPARAMS: unsafe extern "C" fn(
    seL4_TCB,
    seL4_CPtr,
    seL4_Word,
    seL4_Word,
) -> seL4_Error = seL4_TCB_SetSchedParams;
const TCB_SETIPCBUFFER: unsafe extern "C" fn(seL4_TCB, seL4_Word, seL4_CPtr) -> seL4_Error =
    seL4_TCB_SetIPCBuffer;
const TCB_SETSPACE: unsafe extern "C" fn(
    seL4_TCB,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
) -> seL4_Error = seL4_TCB_SetSpace;
const TCB_SUSPEND: unsafe extern "C" fn(seL4_TCB) -> seL4_Error = seL4_TCB_Suspend;
const TCB_RESUME: unsafe extern "C" fn(seL4_TCB) -> seL4_Error = seL4_TCB_Resume;
const TCB_BINDNOTIFICATION: unsafe extern "C" fn(seL4_TCB, seL4_CPtr) -> seL4_Error =
    seL4_TCB_BindNotification;
const TCB_UNBINDNOTIFICATION: unsafe extern "C" fn(seL4_TCB) -> seL4_Error =
    seL4_TCB_UnbindNotification;
const CNODE_REVOKE: unsafe extern "C" fn(seL4_CNode, seL4_Word, seL4_Uint8) -> seL4_Error =
    seL4_CNode_Revoke;
const CNODE_DELETE: unsafe extern "C" fn(seL4_CNode, seL4_Word, seL4_Uint8) -> seL4_Error =
    seL4_CNode_Delete;
const CNODE_CANCELBADGEDSENDS: unsafe extern "C" fn(
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
) -> seL4_Error = seL4_CNode_CancelBadgedSends;
const CNODE_COPY: unsafe extern "C" fn(
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_CapRights,
) -> seL4_Error = seL4_CNode_Copy;
const CNODE_MINT: unsafe extern "C" fn(
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_CapRights,
    seL4_Word,
) -> seL4_Error = seL4_CNode_Mint;
const CNODE_MOVE: unsafe extern "C" fn(
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
) -> seL4_Error = seL4_CNode_Move;
const CNODE_MUTATE: unsafe extern "C" fn(
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_Word,
) -> seL4_Error = seL4_CNode_Mutate;
const CNODE_ROTATE: unsafe extern "C" fn(
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
) -> seL4_Error = seL4_CNode_Rotate;
const CNODE_SAVECALLER: unsafe extern "C" fn(seL4_CNode, seL4_Word, seL4_Uint8) -> seL4_Error =
    seL4_CNode_SaveCaller;
const IRQCONTROL_GET: unsafe extern "C" fn(
    seL4_IRQControl,
    seL4_Word,
    seL4_CNode,
    seL4_Word,
    seL4_Uint8,
) -> seL4_Error = seL4_IRQControl_Get;
const IRQHANDLER_ACK: unsafe extern "C" fn(seL4_IRQHandler) -> seL4_Error = seL4_IRQHandler_Ack;
const IRQHANDLER_SETNOTIFICATION: unsafe extern "C" fn(seL4_IRQHandler, seL4_CPtr) -> seL4_Error =
    seL4_IRQHandler_SetNotification;
const IRQHANDLER_CLEAR: unsafe extern "C" fn(seL4_IRQHandler) -> seL4_Error = seL4_IRQHandler_Clear;
const DOMAINSET_SET: unsafe extern "C" fn(seL4_DomainSet, seL4_Uint8, seL4_TCB) -> seL4_Error =
    seL4_DomainSet_Set;

#[cfg(KernelPrinting)]
mod kernel_printing_compile_time_assertions {
    use super::*;
    const DEBUG_PUT_CHAR: unsafe extern "C" fn(c: ctypes::c_char) = seL4_DebugPutChar;
    const DEBUG_DUMP_SCHEDULER: unsafe extern "C" fn() = seL4_DebugDumpScheduler;
}
#[cfg(DebugBuild)]
mod debug_build_compile_time_assertions {
    use super::*;
    const DEBUG_HALT: unsafe extern "C" fn() = seL4_DebugHalt;
    const DEBUG_SNAPSHOT: unsafe extern "C" fn() = seL4_DebugSnapshot;
    const DEBUG_CAP_IDENTIFY: unsafe extern "C" fn(seL4_CPtr) -> seL4_Uint32 =
        seL4_DebugCapIdentify;
    const DEBUG_NAME_THREAD: unsafe extern "C" fn(seL4_CPtr, *const ctypes::c_char) =
        seL4_DebugNameThread;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_shared_compile_time_assertions {
    use super::*;
    const IRQCONTROL_GETIOAPIC: unsafe extern "C" fn(
        _service: seL4_IRQControl,
        root: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
        ioapic: seL4_Word,
        pin: seL4_Word,
        level: seL4_Word,
        polarity: seL4_Word,
        vector: seL4_Word,
    ) -> seL4_Error = seL4_IRQControl_GetIOAPIC;
    const IRQCONTROL_GETMSI: unsafe extern "C" fn(
        _service: seL4_IRQControl,
        root: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
        pci_bus: seL4_Word,
        pci_dev: seL4_Word,
        pci_func: seL4_Word,
        handle: seL4_Word,
        vector: seL4_Word,
    ) -> seL4_Error = seL4_IRQControl_GetMSI;
    const X86_ASIDCONTROL_MAKEPOOL: unsafe extern "C" fn(
        _service: seL4_X86_ASIDControl,
        untyped: seL4_Untyped,
        root: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
    ) -> seL4_Error = seL4_X86_ASIDControl_MakePool;
    const X86_ASIDPOOL_ASSIGN: unsafe extern "C" fn(
        _service: seL4_X86_ASIDPool,
        vspace: seL4_CPtr,
    ) -> seL4_Error = seL4_X86_ASIDPool_Assign;

    #[cfg(KernelIOMMU)]
    mod iommu_gated {
        use super::super::*;
        const X86_IOPAGETABLE_MAP: unsafe extern "C" fn(
            _service: seL4_X86_IOPageTable,
            iospace: seL4_X86_IOSpace,
            ioaddr: seL4_Word,
        ) -> seL4_Error = seL4_X86_IOPageTable_Map;
        const X86_IOPAGETABLE_UNMAP: unsafe extern "C" fn(
            _service: seL4_X86_IOPageTable,
        ) -> seL4_Error = seL4_X86_IOPageTable_Unmap;
        const X86_PAGE_MAPIO: unsafe extern "C" fn(
            _service: seL4_X86_Page,
            iospace: seL4_X86_IOSpace,
            rights: seL4_CapRights,
            ioaddr: seL4_Word,
        ) -> seL4_Error = seL4_X86_Page_MapIO;
    }

    #[cfg(KernelVTX)]
    mod vtx_gated {
        // TODO seL4_TCB_SetEPTRoot
        // TODO X86_EPTPD functions: seL4_X86_EPTPD_Map, seL4_X86_EPTPD_Unmap
        // TODO X86_EPTPDPT functions: seL4_X86_EPTPDPT_Map, seL4_X86_EPTPDPT_Unmap
        // TODO X86_EPTPT functions: seL4_X86_EPTPT_Map, seL4_X86_EPTPT_Unmap
        // TODO X86_VCPU functions
    }

    #[allow(dead_code)]
    #[doc(hidden)]
    fn x86_extra_paging_structs_exist() {
        let _: seL4_X86_PageDirectory_GetStatusBits;
        let _: seL4_X86_Page_GetAddress;
    }

    const X86_PAGEDIRECTORY_MAP: unsafe extern "C" fn(
        _service: seL4_X86_PageDirectory,
        vspace: seL4_CPtr,
        vaddr: seL4_Word,
        attr: seL4_X86_VMAttributes,
    ) -> seL4_Error = seL4_X86_PageDirectory_Map;

    const X86_PAGEDIRECTORY_UNMAP: unsafe extern "C" fn(
        _service: seL4_X86_PageDirectory,
    ) -> seL4_Error = seL4_X86_PageDirectory_Unmap;

    const X86_PAGETABLE_MAP: unsafe extern "C" fn(
        _service: seL4_X86_PageTable,
        vspace: seL4_CPtr,
        vaddr: seL4_Word,
        attr: seL4_X86_VMAttributes,
    ) -> seL4_Error = seL4_X86_PageTable_Map;

    const X86_PAGETABLE_UNMAP: unsafe extern "C" fn(_service: seL4_X86_PageTable) -> seL4_Error =
        seL4_X86_PageTable_Unmap;

    const X86_PAGE_MAP: unsafe extern "C" fn(
        _service: seL4_X86_Page,
        vspace: seL4_CPtr,
        vaddr: seL4_Word,
        rights: seL4_CapRights_t,
        attr: seL4_X86_VMAttributes,
    ) -> seL4_Error = seL4_X86_Page_Map;

    const X86_PAGE_UNMAP: unsafe extern "C" fn(_service: seL4_X86_Page) -> seL4_Error =
        seL4_X86_Page_Unmap;

    const X86_PAGE_GETADDRESS: unsafe extern "C" fn(
        _service: seL4_X86_Page,
    ) -> seL4_X86_Page_GetAddress_t = seL4_X86_Page_GetAddress;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn x86_extra_io_structs_exist() {
        let _: seL4_X86_IOPort_In8_t;
        let _: seL4_X86_IOPort_In16_t;
        let _: seL4_X86_IOPort_In32_t;
    }

    const X86_IOPORTCONTROL_ISSUE: unsafe extern "C" fn(
        _service: seL4_X86_IOPortControl,
        first_port: seL4_Word,
        last_port: seL4_Word,
        root: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
    ) -> seL4_Error = seL4_X86_IOPortControl_Issue;

    const X86_IOPORT_IN8: unsafe extern "C" fn(
        _service: seL4_X86_IOPort,
        port: seL4_Uint16,
    ) -> seL4_X86_IOPort_In8_t = seL4_X86_IOPort_In8;

    const X86_IOPORT_IN16: unsafe extern "C" fn(
        _service: seL4_X86_IOPort,
        port: seL4_Uint16,
    ) -> seL4_X86_IOPort_In16_t = seL4_X86_IOPort_In16;

    const X86_IOPORT_IN32: unsafe extern "C" fn(
        _service: seL4_X86_IOPort,
        port: seL4_Uint16,
    ) -> seL4_X86_IOPort_In32_t = seL4_X86_IOPort_In32;

    const X86_IOPORT_OUT8: unsafe extern "C" fn(
        _service: seL4_X86_IOPort,
        port: seL4_Word,
        data: seL4_Word,
    ) -> seL4_Error = seL4_X86_IOPort_Out8;

    const X86_IOPORT_OUT16: unsafe extern "C" fn(
        _service: seL4_X86_IOPort,
        port: seL4_Word,
        data: seL4_Word,
    ) -> seL4_Error = seL4_X86_IOPort_Out16;

    const X86_IOPORT_OUT32: unsafe extern "C" fn(
        _service: seL4_X86_IOPort,
        port: seL4_Word,
        data: seL4_Word,
    ) -> seL4_Error = seL4_X86_IOPort_Out32;
}

#[cfg(target_arch = "x86_64")]
mod x86_64_specific_compile_time_assertions {
    use super::*;
    const X86_PDPT_MAP: unsafe extern "C" fn(
        _service: seL4_X86_PDPT,
        pml4: seL4_X64_PML4,
        vaddr: seL4_Word,
        attr: seL4_X86_VMAttributes,
    ) -> seL4_Error = seL4_X86_PDPT_Map;
    const X86_PDPT_UNMAP: unsafe extern "C" fn(_service: seL4_X86_PDPT) -> seL4_Error =
        seL4_X86_PDPT_Unmap;
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64", target_arch = "aarch32"))]
mod arm_specific_compile_time_assertions {
    use super::*;
    const ARM_ASIDCONTROL_MAKEPOOL: unsafe extern "C" fn(
        _service: seL4_ARM_ASIDControl,
        untyped: seL4_Untyped,
        root: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
    ) -> seL4_Error = seL4_ARM_ASIDControl_MakePool;
    const ARM_ASIDPOOL_ASSIGN: unsafe extern "C" fn(
        _service: seL4_ARM_ASIDPool,
        vspace: seL4_CPtr,
    ) -> seL4_Error = seL4_ARM_ASIDPool_Assign;
    const IRQCONTROL_GETTRIGGER: unsafe extern "C" fn(
        _service: seL4_IRQControl,
        irq: seL4_Word,
        trigger: seL4_Word,
        root: seL4_CNode,
        index: seL4_Word,
        depth: seL4_Uint8,
    ) -> seL4_Error = seL4_IRQControl_GetTrigger;

    const ARM_PAGE_CLEAN_DATA: unsafe extern "C" fn(
        _service: seL4_ARM_Page,
        start_offset: seL4_Word,
        end_offset: seL4_Word,
    ) -> seL4_Error = seL4_ARM_Page_Clean_Data;
    const ARM_PAGE_CLEANINVALIDATE_DATA: unsafe extern "C" fn(
        _service: seL4_ARM_Page,
        start_offset: seL4_Word,
        end_offset: seL4_Word,
    ) -> seL4_Error = seL4_ARM_Page_CleanInvalidate_Data;
    const ARM_PAGE_GETADDRESS: unsafe extern "C" fn(
        _service: seL4_ARM_Page,
    ) -> seL4_ARM_Page_GetAddress_t = seL4_ARM_Page_GetAddress;
    const ARM_PAGE_INVALIDATE_DATA: unsafe extern "C" fn(
        _service: seL4_ARM_Page,
        start_offset: seL4_Word,
        end_offset: seL4_Word,
    ) -> seL4_Error = seL4_ARM_Page_Invalidate_Data;
    const ARM_PAGE_MAP: unsafe extern "C" fn(
        _service: seL4_ARM_Page,
        vspace: seL4_CPtr,
        vaddr: seL4_Word,
        rights: seL4_CapRights_t,
        attr: seL4_ARM_VMAttributes,
    ) -> seL4_Error = seL4_ARM_Page_Map;
    const ARM_PAGE_UNIFY_INSTRUCTION: unsafe extern "C" fn(
        _service: seL4_ARM_Page,
        start_offset: seL4_Word,
        end_offset: seL4_Word,
    ) -> seL4_Error = seL4_ARM_Page_Unify_Instruction;
    const ARM_PAGE_UNMAP: unsafe extern "C" fn(_service: seL4_ARM_Page) -> seL4_Error =
        seL4_ARM_Page_Unmap;

    const ARM_PAGETABLE_MAP: unsafe extern "C" fn(
        _service: seL4_ARM_PageTable,
        vspace: seL4_CPtr,
        vaddr: seL4_Word,
        attr: seL4_ARM_VMAttributes,
    ) -> seL4_Error = seL4_ARM_PageTable_Map;
    const ARM_PAGETABLE_UNMAP: unsafe extern "C" fn(_service: seL4_ARM_PageTable) -> seL4_Error =
        seL4_ARM_PageTable_Unmap;

    #[cfg(KernelArmSMMU)]
    mod smmu_gated {
        use super::super::*;
        const ARM_IOPAGETABLE_MAP: unsafe extern "C" fn(
            _service: seL4_ARM_IOPageTable,
            iospace: seL4_ARM_IOSpace,
            ioaddr: seL4_Word,
        ) -> seL4_Error = seL4_ARM_IOPageTable_Map;
        const ARM_IOPAGETABLE_UNMAP: unsafe extern "C" fn(
            _service: seL4_ARM_IOPageTable,
        ) -> seL4_Error = seL4_ARM_IOPageTable_Unmap;
        const ARM_PAGE_MAPIO: unsafe extern "C" fn(
            _service: seL4_ARM_Page,
            iospace: seL4_ARM_IOSpace,
            rights: seL4_CapRights_t,
            ioaddr: seL4_Word,
        ) -> seL4_Error = seL4_ARM_Page_MapIO;
    }

    #[cfg(KernelArmHypervisorSupport)]
    mod hyp_gated {
        use super::super::*;
        const ARM_VCPU_INJECTIRQ: unsafe extern "C" fn(
            _service: seL4_ARM_VCPU,
            virq: seL4_Uint16,
            priority: seL4_Uint8,
            group: seL4_Uint8,
            index: seL4_Uint8,
        ) -> seL4_Error = seL4_ARM_VCPU_InjectIRQ;
        const ARM_VCPU_READREGS: unsafe extern "C" fn(
            _service: seL4_ARM_VCPU,
            field: seL4_Word,
        ) -> seL4_ARM_VCPU_ReadRegs_t = seL4_ARM_VCPU_ReadRegs;
        const ARM_VCPU_SETTCB: unsafe extern "C" fn(
            _service: seL4_ARM_VCPU,
            tcb: seL4_TCB,
        ) -> seL4_Error = seL4_ARM_VCPU_SetTCB;
        const ARM_VCPU_WRITEREGS: unsafe extern "C" fn(
            _service: seL4_ARM_VCPU,
            field: seL4_Word,
            value: seL4_Word,
        ) -> seL4_Error = seL4_ARM_VCPU_WriteRegs;
    }

    #[cfg(target_pointer_width = "32")]
    mod thirty_two_bit_specific {
        use super::super::*;
        const ARM_PAGEDIRECTORY_CLEAN_DATA: unsafe extern "C" fn(
            _service: seL4_ARM_PageDirectory,
            start: seL4_Word,
            end: seL4_Word,
        ) -> seL4_Error = seL4_ARM_PageDirectory_Clean_Data;
        const ARM_PAGEDIRECTORY_INVALIDATE_DATA: unsafe extern "C" fn(
            _service: seL4_ARM_PageDirectory,
            start: seL4_Word,
            end: seL4_Word,
        ) -> seL4_Error = seL4_ARM_PageDirectory_Invalidate_Data;
        const ARM_PAGEDIRECTORY_CLEANINVALIDATE_DATA: unsafe extern "C" fn(
            _service: seL4_ARM_PageDirectory,
            start: seL4_Word,
            end: seL4_Word,
        ) -> seL4_Error = seL4_ARM_PageDirectory_CleanInvalidate_Data;
        const ARM_PAGEDIRECTORY_UNIFY_INSTRUCTION: unsafe extern "C" fn(
            _service: seL4_ARM_PageDirectory,
            start: seL4_Word,
            end: seL4_Word,
        ) -> seL4_Error = seL4_ARM_PageDirectory_Unify_Instruction;
    }

    #[cfg(target_pointer_width = "64")]
    mod sixty_four_bit_specific {
        // TODO - 64 bit specific functions
    }
}
// TODO - add presence-checks for more constants of interest, e.g. the retype-ids for arch-agnostic kernel objects
