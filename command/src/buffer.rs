//! Buffer module docs.

use relevant::Relevant;
use std::fmt::Debug;

use device::CommandBuffer;
use encoder::Encoder;
use family::FamilyId;
use frame::FrameBound;

/// Command buffers of this level can be submitted to the command queues.
#[derive(Clone, Copy, Debug)]
pub struct PrimaryLevel;

/// Command buffers of this level can be executed as part of the primary buffers.
#[derive(Clone, Copy, Debug)]
pub struct SecondaryLevel;

/// This flag specify that buffer can be reset individually.
/// Without this flag buffer can be reset only together with all other buffers from pool.
#[derive(Clone, Copy, Debug)]
pub struct IndividualReset;

/// Command buffer state in which all buffers start.
/// Resetting also moves buffer to this state.
#[derive(Clone, Copy, Debug)]
pub struct InitialState;

/// Command buffer in recording state could be populated with commands.
#[derive(Clone, Copy, Debug)]
pub struct RecordingState<U>(U);

/// Command buffer in executable state can be submitted.
#[derive(Clone, Copy, Debug)]
pub struct ExecutableState<U>(U);

/// Command buffer in pending state are submitted to the device.
/// Buffer in pending state must never be invalidated or reset because device may read it at the moment.
/// Proving device is done with buffer requires nontrivial strategies.
/// Therefore moving buffer from pending state requires `unsafe` method.
#[derive(Clone, Copy, Debug)]
pub struct PendingState<N>(N);

/// One-shot buffers move to invalid state after execution.
/// Invalidating any resource referenced in any command recorded to the buffer implicitly move it to the invalid state.
#[derive(Clone, Copy, Debug)]
pub struct InvalidState;

/// States in which command buffer can be destroyed.
pub trait Droppable {}
impl Droppable for InitialState {}
impl<U> Droppable for RecordingState<U> {}
impl<U> Droppable for ExecutableState<U> {}
impl Droppable for InvalidState {}

/// States in which command buffer can de reset.
pub trait Resettable: Droppable {}
impl<U> Resettable for RecordingState<U> {}
impl<U> Resettable for ExecutableState<U> {}
impl Resettable for InvalidState {}

/// Buffer with this usage flag will move to invalid state after execution.
/// Resubmitting will require reset and rerecording commands.
#[derive(Clone, Copy, Debug)]
pub struct OneShot;

/// Buffer with this usage flag will move back to executable state after execution.
#[derive(Clone, Copy, Debug)]
pub struct MultiShot<S = ()>(S);

/// Additional flag for `MultiShot` that allows to resubmit buffer in pending state.
/// Note that resubmitting pending buffers can hurt performance.
#[derive(Clone, Copy, Debug)]
pub struct SimultaneousUse;

/// Buffers with this usage flag must be secondary buffers executed entirely in render-pass.
#[derive(Clone, Copy, Debug)]
pub struct RenderPassContinue;

bitflags!{
    /// Bitmask specifying usage behavior for command buffer
    /// See Vulkan docs for detailed info:
    /// <https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferUsageFlagBits.html>
    #[repr(transparent)]
    pub struct UsageFlags: u32 {
        /// Specifies that each recording of the command buffer will only be submitted once,
        /// and the command buffer will be reset and recorded again between each submission.
        const ONE_TIME_SUBMIT = 0x00000001;

        /// Specifies that a secondary command buffer is considered to be entirely inside a render pass.
        /// If this is a primary command buffer, then this bit is ignored.
        const RENDER_PASS_CONTINUE = 0x00000002;

        /// Specifies that a command buffer can be resubmitted to a queue while it is in the pending state,
        /// and recorded into multiple primary command buffers.
        const SIMULTANEOUS_USE = 0x00000004;
    }
}

/// Trait implemented by all usage types.
pub trait Usage {
    /// State in which command buffer moves after completion.

    fn flags(&self) -> UsageFlags;
}

impl Usage for OneShot {
    fn flags(&self) -> UsageFlags {
        UsageFlags::ONE_TIME_SUBMIT
    }
}

impl Usage for MultiShot {
    fn flags(&self) -> UsageFlags {
        UsageFlags::empty()
    }
}

impl Usage for MultiShot<SimultaneousUse> {
    fn flags(&self) -> UsageFlags {
        UsageFlags::SIMULTANEOUS_USE
    }
}

/// Command buffer wrapper.
/// This wrapper defines state with usage, level and ability to be individually reset at type level.
/// This way many methods become safe.
#[derive(Debug)]
pub struct Buffer<B, C, S, L, R = ()> {
    inner: B,
    capability: C,
    state: S,
    level: L,
    reset: R,
    family: FamilyId,
    relevant: Relevant,
}

impl<B, C, R> Buffer<B, C, InitialState, PrimaryLevel, R> {
    /// Begin recording command buffer.
    ///
    /// # Parameters
    ///
    /// `usage` - specifies usage of the command buffer. Possible types are `OneShot`, `MultiShot`.
    pub fn begin<U>(self, usage: U) -> Buffer<B, C, RecordingState<U>, PrimaryLevel, R>
    where
        U: Usage,
    {
        unimplemented!()
    }
}

/// Structure contains command buffer ready for submission.
#[derive(Debug)]
pub struct Submit<S> {
    raw: S,
    family: FamilyId,
}

impl<S> Submit<S> {
    /// Get family this submit is associated with.
    pub fn family(&self) -> FamilyId {
        self.family
    }

    /// Unwrap inner submit value.
    pub fn into_inner(self) -> S {
        self.raw
    }
}

impl<B, C, R> Buffer<B, C, ExecutableState<OneShot>, PrimaryLevel, R>
where
    B: CommandBuffer,
{
    /// produce `Submit` object that can be used to populate submission.
    pub fn submit_once(
        self,
    ) -> (
        Submit<B::Submit>,
        Buffer<B, C, PendingState<InvalidState>, PrimaryLevel, R>,
    ) {
        unimplemented!()
    }
}

impl<B, C, S, R> Buffer<B, C, ExecutableState<MultiShot<S>>, PrimaryLevel, R>
where
    B: CommandBuffer,
{
    /// Produce `Submit` object that can be used to populate submission.
    pub fn submit(
        self,
    ) -> (
        Submit<B::Submit>,
        Buffer<B, C, PendingState<ExecutableState<MultiShot<S>>>, PrimaryLevel, R>,
    ) {
        unimplemented!()
    }
}

impl<B, C, N, L, R> Buffer<B, C, PendingState<N>, L, R> {
    /// Mark command buffer as complete.
    ///
    /// # Safety
    ///
    /// User must ensure that recorded commands are complete.
    pub unsafe fn complete(self) -> Buffer<B, C, N, L, R> {
        unimplemented!()
    }
}

impl<B, C, S, L> Buffer<B, C, S, L, IndividualReset>
where
    S: Resettable,
{
    /// Reset command buffer.
    pub fn reset(self) -> Buffer<B, C, InitialState, L, IndividualReset> {
        unimplemented!()
    }
}

impl<B, C, S, L> Buffer<B, C, S, L>
where
    S: Resettable,
{
    /// Reset command buffer.
    ///
    /// # Safety
    ///
    /// Mark command buffer as reset.
    /// User must reset buffer via command pool and call this method for all commands buffers affected.
    pub unsafe fn mark_reset(self) -> Buffer<B, C, InitialState, L> {
        unimplemented!()
    }
}

impl<B, C, U, L, R> Encoder<C> for Buffer<B, C, RecordingState<U>, L, R>
where
    B: CommandBuffer,
{
    type Buffer = B;

    unsafe fn buffer(&mut self) -> &mut B {
        &mut self.inner
    }
}

impl<'a, F: 'a, B> CommandBuffer for FrameBound<'a, F, B>
where
    B: CommandBuffer,
    F: Debug,
{
    type Submit = FrameBound<'a, F, B::Submit>;

    unsafe fn submit(&self) -> FrameBound<'a, F, B::Submit> {
        FrameBound::bind(self.inner_ref().submit(), self.frame())
    }
}

impl<'a, F: 'a, B, S, L, C> Buffer<FrameBound<'a, F, B>, C, S, L> {
    /// Release borrowed buffer. This allows to acquire next buffer from pool.
    /// Whatever state this buffer was in it will be reset only after bounded frame is complete.
    /// This allows safely to release borrowed buffer in pending state.
    pub fn release(self) {
        unimplemented!()
    }
}
