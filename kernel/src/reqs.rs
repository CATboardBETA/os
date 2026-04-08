#![allow(unused)]
use crate::kmain;
use limine::request::BootloaderInfoRequest;
use limine::request::BootloaderPerformanceRequest;
#[cfg(target_arch = "riscv64")]
use limine::request::BspHartidRequest;
use limine::request::DateAtBootRequest;
use limine::request::DtbRequest;
use limine::request::EfiMemmapRequest;
use limine::request::EfiRequest;
use limine::request::EntryPointRequest;
use limine::request::ExecutableAddressRequest;
use limine::request::ExecutableCmdlineRequest;
use limine::request::ExecutableFileRequest;
use limine::request::FirmwareTypeRequest;
use limine::request::FramebufferRequest;
use limine::request::HhdmRequest;
#[cfg(target_arch = "x86_64")]
use limine::request::KeepIommuRequest;
use limine::request::MemmapRequest;
use limine::request::ModulesRequest;
use limine::request::MpRequest;
use limine::request::PagingModeRequest;
use limine::request::RsdpRequest;
use limine::request::SmbiosRequest;
use limine::request::StackSizeRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: BaseRevision = BaseRevision::with_revision(6);
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();
#[unsafe(link_section = ".requests")]
pub static MEMMAP: MemmapRequest = MemmapRequest::new();
#[unsafe(link_section = ".requests")]
pub static BOOTLOADER: BootloaderInfoRequest = BootloaderInfoRequest::new();
#[unsafe(link_section = ".requests")]
pub static FIRMWARE: FirmwareTypeRequest = FirmwareTypeRequest::new();
#[unsafe(link_section = ".requests")]
pub static DATE: DateAtBootRequest = DateAtBootRequest::new();
#[unsafe(link_section = ".requests")]
pub static BOOT_TIME: BootloaderPerformanceRequest = BootloaderPerformanceRequest::new();
#[unsafe(link_section = ".requests")]
pub static HHDM: HhdmRequest = HhdmRequest::new();
#[unsafe(link_section = ".requests")]
pub static EXEC_ADDR: ExecutableAddressRequest = ExecutableAddressRequest::new();
#[unsafe(link_section = ".requests")]
pub static EXEC_FILE: ExecutableFileRequest = ExecutableFileRequest::new();
#[unsafe(link_section = ".requests")]
pub static EXEC_CMDLINE: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();
#[unsafe(link_section = ".requests")]
pub static DTB: DtbRequest = DtbRequest::new();
#[unsafe(link_section = ".requests")]
pub static RSDP: RsdpRequest = RsdpRequest::new();
#[unsafe(link_section = ".requests")]
pub static MP: MpRequest = MpRequest::new(0);
#[cfg(target_arch = "riscv64")]
#[unsafe(link_section = ".requests")]
pub static BSP_HARTID: BspHartidRequest = BspHartidRequest::new();
#[unsafe(link_section = ".requests")]
pub static MODULES: ModulesRequest = ModulesRequest::new();
#[cfg(target_arch = "x86_64")]
#[unsafe(link_section = ".requests")]
pub static KEEP_IOMMU: KeepIommuRequest = KeepIommuRequest::new();
#[unsafe(link_section = ".requests")]
pub static STACK: StackSizeRequest = StackSizeRequest::new(65536);
#[unsafe(link_section = ".requests")]
pub static PAGING: PagingModeRequest = PagingModeRequest::PREFER_MAXIMUM;
#[unsafe(link_section = ".requests")]
pub static ENTRY: EntryPointRequest = EntryPointRequest::new(kmain);
#[unsafe(link_section = ".requests")]
pub static SMBIOS: SmbiosRequest = SmbiosRequest::new();
#[unsafe(link_section = ".requests")]
pub static EFI: EfiRequest = EfiRequest::new();
#[unsafe(link_section = ".requests")]
pub static EFI_MEMMAP: EfiMemmapRequest = EfiMemmapRequest::new();
#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();
