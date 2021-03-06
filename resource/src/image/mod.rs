//! Image usage, format, kind, extent, creation-info and wrappers.

pub mod format;
mod usage;

pub use self::format::Format;
pub use self::usage::*;

use memory::MemoryBlock;
use relevant::Relevant;

use escape::Escape;
use SharingMode;

/// Image dimensionality
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Image with single dimension. A line.
    D1,

    /// Two-dimensional image. Most widely used image kind.
    D2,

    /// Full 3D image. Can represent volumetric textures.
    D3,
}

/// Image size. Unused dimensions must have size `1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extent3D {
    /// The width of the extent
    pub width: u32,
    /// The height of the extent
    pub height: u32,
    /// The depth of the extent
    pub depth: u32,
}

bitflags! {
    /// Bitmask specifying sample counts supported for an image used for storage operations.
    /// See Vulkan docs for detailed info:
    /// <https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSampleCountFlagBits.html>
    #[repr(transparent)]
    pub struct SampleCountFlags: u32 {
        /// Specifies an image with one sample per pixel.
        const SAMPLE_COUNT_1 = 0x00000001;
        /// Specifies an image with 2 sample per pixel.
        const SAMPLE_COUNT_2 = 0x00000002;
        /// Specifies an image with 4 sample per pixel.
        const SAMPLE_COUNT_4 = 0x00000004;
        /// Specifies an image with 8 sample per pixel.
        const SAMPLE_COUNT_8 = 0x00000008;
        /// Specifies an image with 16 sample per pixel.
        const SAMPLE_COUNT_16 = 0x00000010;
        /// Specifies an image with 32 sample per pixel.
        const SAMPLE_COUNT_32 = 0x00000020;
        /// Specifies an image with 64 sample per pixel.
        const SAMPLE_COUNT_64 = 0x00000040;
    }
}

/// Image tiling type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageTiling {
    /// Implementation-defined tiling mode. Texels are arranged for more optimal memory access.
    Optimal = 0,

    /// Texels are laid in row-major order.
    Linear = 1,
}

/// Image layout.
/// Different layouts support different sets of device accesses.
/// See Vulkan docs for details:
/// <https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/vkspec.html#resources-image-layouts>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// Not an actual layout.
    /// It can be used as source layout in layout transition
    /// in which case transition is no-op and image is just
    /// interpreted to have destination layout.
    /// No other operations can be performed with this layout.
    Undefined = 0,

    /// Supports all types of device access.
    /// But access could be sub-optimal.
    General = 1,

    /// Images with this layout can be used as color and resolve attachments.
    ColorAttachmentOptimal = 2,

    /// Images with this layout can be used as depth-stencil attachments.
    DepthStencilAttachmentOptimal = 3,

    /// Images with this layout can be used as read-only depth-stencil attachments
    /// or as read-only image in shader.
    DepthStencilReadOnlyOptimal = 4,

    /// Images with this layout can be used as read-only shader image.
    ShaderReadOnlyOptimal = 5,

    /// Images with this layout can be used as source for transfer operations.
    TransferSrcOptimal = 6,

    /// Images with this layout can be used as destination for transfer operations.
    TransferDstOptimal = 7,

    /// Image in this layout can be transitioned to another layout while preserving content.
    /// This layout usable as initial layout for image which content will be written by the host.
    Preinitialized = 8,

    /// Images with this layout can be used as depth-stencil attachments where
    /// depth aspect is read-only and/or as read-only image in shader where only depth aspect is accessed.
    DepthReadOnlyStencilAttachmentOptimal = 1000117000,

    /// Images with this layout can be used as depth-stencil attachments where
    /// stencil aspect is read-only and/or as read-only image in shader where only stencil aspect is accessed.
    DepthAttachmentStencilReadOnlyOptimal = 1000117001,

    /// Image with this layout can be presented to the surface.
    /// Only images from swapchain are presentable.
    /// Note: Images can't be presented in `General` layout.
    PresentSrc = 1000001002,

    /// This layout is only valid for shared presentable images.
    /// They can be used for any operations such image supports.
    SharedPresentSrc = 1000111000,
}

bitflags! {
    /// Bitmask specifying capabilities to create views into an image.
    /// See Vulkan docs for details:
    /// <https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/vkspec.html#VkImageCreateFlagBits>
    #[repr(transparent)]
    pub struct ImageCreateFlags: u32 {
        /// Specifies that the image can be used to create a view with a different format from the image.
        const IMAGE_CREATE_MUTABLE_FORMAT = 0x00000008;
        /// Specifies that the image can be used to create a cube or cube array view.
        const IMAGE_CREATE_CUBE_COMPATIBLE = 0x00000010;
        /// Specifies that the image can be used to create a 2D array view.
        const IMAGE_CREATE_2D_ARRAY_COMPATIBLE = 0x00000020;
    }
}

/// Contains information required to create an image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CreateInfo {
    /// Image dimensionality.
    pub kind: Kind,

    /// Image format.
    pub format: Format,

    /// Image size.
    pub extent: Extent3D,

    /// Number of mip levels to generate.
    pub mips: u32,

    /// Number of image layers.
    pub array: u32,

    /// Number of samples per texel.
    pub samples: SampleCountFlags,

    /// Tiling of the image.
    pub tiling: ImageTiling,

    /// Intended usage flags. Limits memory types suitable for the image.
    pub usage: UsageFlags,

    /// Specifies command queues from which families can access the image.
    pub sharing: SharingMode,

    /// Specifies what kind of views can be created from the image.
    pub flags: ImageCreateFlags,
}

/// Generic image object wrapper.
///
/// # Parameters
///
/// `T` - type of the memory object of memory block.
/// `B` - raw image type.
#[derive(Debug)]
pub struct Image<M, I> {
    pub(super) inner: Escape<Inner<M, I>>,
    pub(super) info: CreateInfo,
}

#[derive(Debug)]
pub(super) struct Inner<M, I> {
    pub(super) block: MemoryBlock<M>,
    pub(super) raw: I,
    pub(super) relevant: Relevant,
}
