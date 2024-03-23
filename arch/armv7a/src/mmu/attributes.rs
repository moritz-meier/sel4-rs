/// Default device memory (Read-only, Never-execute, Privilege-level 1)
pub const DEVICE: MemoryAttributes = MemoryAttributes {
    typ: MemoryType::Device,
    access: MemoryAccess {
        read_write: ReadWritePolicy::ReadOnly,
        execute: ExecutePolicy::ExecuteNever,
        privilege: PrivilegeLevel::PrivilegeLevel1,
    },
};

/// Default strongly-ordered memory (Read-only, Never-execute, Privilege-level 1)
pub const STRONGLY_ORDERED: MemoryAttributes = MemoryAttributes {
    typ: MemoryType::StronglyOrdered,
    access: MemoryAccess {
        read_write: ReadWritePolicy::ReadOnly,
        execute: ExecutePolicy::ExecuteNever,
        privilege: PrivilegeLevel::PrivilegeLevel1,
    },
};

/// Default normal memory (Non-cacheable, Non-shareable, Read-only, Never-execute, Privilege-level 1)
pub const NORMAL: MemoryAttributes = MemoryAttributes {
    typ: MemoryType::Normal {
        inner: CachePolicy::NonCacheable,
        outer: CachePolicy::NonCacheable,
        share: SharePolicy::NonShareable,
    },
    access: MemoryAccess {
        read_write: ReadWritePolicy::ReadOnly,
        execute: ExecutePolicy::ExecuteNever,
        privilege: PrivilegeLevel::PrivilegeLevel1,
    },
};

#[derive(Clone, Copy)]
pub struct MemoryAttributes {
    typ: MemoryType,
    access: MemoryAccess,
}

impl MemoryAttributes {
    /// Set inner cache policy. Only affects Normal memory.
    pub const fn inner(self, inner: CachePolicy) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner: _,
                outer,
                share,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Set outer cache policy. Only affects Normal memory.
    pub const fn outer(self, outer: CachePolicy) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner,
                outer: _,
                share,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Mark memory as non-shareable. Only affects Normal memory.
    pub const fn non_shareable(self) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner,
                outer,
                share: _,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share: SharePolicy::NonShareable,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Mark memory as shareable. Only affects Normal memory.
    pub const fn shareable(self) -> Self {
        match self.typ {
            MemoryType::Normal {
                inner,
                outer,
                share: _,
            } => Self {
                typ: MemoryType::Normal {
                    inner,
                    outer,
                    share: SharePolicy::Shareable,
                },
                ..self
            },
            _ => self,
        }
    }

    /// Mark memory as read-only.
    pub const fn read_only(self) -> Self {
        Self {
            access: self.access.read_only(),
            ..self
        }
    }

    /// Mark memory as read-writeable.
    pub const fn read_writeable(self) -> Self {
        Self {
            access: self.access.read_writeable(),
            ..self
        }
    }

    /// Mark memory as not-executable.
    pub const fn execute_never(self) -> Self {
        Self {
            access: self.access.execute_never(),
            ..self
        }
    }

    /// Mark memory as executeable.
    pub const fn executeable(self) -> Self {
        Self {
            access: self.access.executeable(),
            ..self
        }
    }

    /// Mark memory as accessible by privilege level 0 (and 1).
    pub const fn privilege_level_0(self) -> Self {
        Self {
            access: self.access.privilege_level_0(),
            ..self
        }
    }

    /// Mark memory as accessible only by privilege level 1.
    pub const fn privilege_level_1(self) -> Self {
        Self {
            access: self.access.privilege_level_1(),
            ..self
        }
    }
}

// ----------------------------------------------

#[derive(Clone, Copy)]
pub enum MemoryType {
    Device,
    StronglyOrdered,
    Normal {
        inner: CachePolicy,
        outer: CachePolicy,
        share: SharePolicy,
    },
}

#[derive(Clone, Copy)]
pub enum CachePolicy {
    NonCacheable,
    WriteBackWriteAllocate,
    WriteThroughNoWriteAllocate,
    WriteBackNoWriteAllocate,
}

#[derive(Clone, Copy)]
pub enum SharePolicy {
    NonShareable,
    Shareable,
}

// ----------------------------------------------

#[derive(Clone, Copy)]
pub struct MemoryAccess {
    read_write: ReadWritePolicy,
    execute: ExecutePolicy,
    privilege: PrivilegeLevel,
}

impl MemoryAccess {
    const fn read_only(self) -> Self {
        Self {
            read_write: ReadWritePolicy::ReadOnly,
            ..self
        }
    }

    const fn read_writeable(self) -> Self {
        Self {
            read_write: ReadWritePolicy::ReadWrite,
            ..self
        }
    }

    const fn execute_never(self) -> Self {
        Self {
            execute: ExecutePolicy::ExecuteNever,
            ..self
        }
    }

    const fn executeable(self) -> Self {
        Self {
            execute: ExecutePolicy::Executeable,
            ..self
        }
    }

    const fn privilege_level_0(self) -> Self {
        Self {
            privilege: PrivilegeLevel::PrivilegeLevel0,
            ..self
        }
    }

    const fn privilege_level_1(self) -> Self {
        Self {
            privilege: PrivilegeLevel::PrivilegeLevel1,
            ..self
        }
    }
}

#[derive(Clone, Copy)]
pub enum ReadWritePolicy {
    ReadOnly,
    ReadWrite,
}

#[derive(Clone, Copy)]
pub enum ExecutePolicy {
    ExecuteNever,
    Executeable,
}

#[derive(Clone, Copy)]
pub enum PrivilegeLevel {
    PrivilegeLevel1,
    PrivilegeLevel0,
}

///------------------------------------------------------------
use crate::registers::*;

use super::bitfields::*;

pub trait FieldEncoding<R: RegisterLongName> {
    fn encode(&self) -> FieldValue<u32, R>;
}

struct InnerEncoding;
struct OuterEncoding;

trait CachePolicyEncoding<InnerOuter, R: RegisterLongName> {
    fn encode(&self) -> FieldValue<u32, R>;
}

///------------------------------------------------------------
/// Impl for Section::Register

impl FieldEncoding<Section::Register> for MemoryAttributes {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        self.typ.encode() + self.access.encode()
    }
}

impl FieldEncoding<Section::Register> for MemoryType {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            MemoryType::Device => Section::TEX::DeviceOrStronglyOrdered + Section::CB::Device,
            MemoryType::StronglyOrdered => {
                Section::TEX::DeviceOrStronglyOrdered + Section::CB::StronglyOrdered
            }
            MemoryType::Normal {
                inner,
                outer,
                share,
            } => {
                Section::TEX::Normal
                    + CachePolicyEncoding::<InnerEncoding, Section::Register>::encode(inner)
                    + CachePolicyEncoding::<OuterEncoding, Section::Register>::encode(outer)
                    + share.encode()
            }
        }
    }
}

impl CachePolicyEncoding<InnerEncoding, Section::Register> for CachePolicy {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            CachePolicy::NonCacheable => Section::AA::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => Section::AA::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => Section::AA::WriteThroughNoWriteAllocate,
            CachePolicy::WriteBackNoWriteAllocate => Section::AA::WriteBackNoWriteAllocate,
        }
    }
}

impl CachePolicyEncoding<OuterEncoding, Section::Register> for CachePolicy {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            CachePolicy::NonCacheable => Section::BB::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => Section::BB::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => Section::BB::WriteThroughNoWriteAllocate,
            CachePolicy::WriteBackNoWriteAllocate => Section::BB::WriteBackNoWriteAllocate,
        }
    }
}

impl FieldEncoding<Section::Register> for SharePolicy {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            SharePolicy::NonShareable => Section::S::NonShareable,
            SharePolicy::Shareable => Section::S::Shareable,
        }
    }
}

impl FieldEncoding<Section::Register> for MemoryAccess {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        self.read_write.encode() + self.execute.encode() + self.privilege.encode()
    }
}

impl FieldEncoding<Section::Register> for ReadWritePolicy {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            ReadWritePolicy::ReadOnly => Section::APX::DisableWrite,
            ReadWritePolicy::ReadWrite => Section::APX::EnableWrite,
        }
    }
}

impl FieldEncoding<Section::Register> for ExecutePolicy {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            ExecutePolicy::ExecuteNever => Section::XN::ExecuteNever,
            ExecutePolicy::Executeable => Section::XN::Executeable,
        }
    }
}

impl FieldEncoding<Section::Register> for PrivilegeLevel {
    fn encode(&self) -> FieldValue<u32, Section::Register> {
        match self {
            PrivilegeLevel::PrivilegeLevel0 => Section::AP::PL0,
            PrivilegeLevel::PrivilegeLevel1 => Section::AP::PL1,
        }
    }
}

///------------------------------------------------------------
/// Impl for Page::Register

impl FieldEncoding<Page::Register> for MemoryAttributes {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        self.typ.encode() + self.access.encode()
    }
}

impl FieldEncoding<Page::Register> for MemoryType {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            MemoryType::Device => Page::TEX::DeviceOrStronglyOrdered + Page::CB::Device,
            MemoryType::StronglyOrdered => {
                Page::TEX::DeviceOrStronglyOrdered + Page::CB::StronglyOrdered
            }
            MemoryType::Normal {
                inner,
                outer,
                share,
            } => {
                Page::TEX::Normal
                    + CachePolicyEncoding::<InnerEncoding, Page::Register>::encode(inner)
                    + CachePolicyEncoding::<OuterEncoding, Page::Register>::encode(outer)
                    + share.encode()
            }
        }
    }
}

impl CachePolicyEncoding<InnerEncoding, Page::Register> for CachePolicy {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            CachePolicy::NonCacheable => Page::AA::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => Page::AA::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => Page::AA::WriteThroughNoWriteAllocate,
            CachePolicy::WriteBackNoWriteAllocate => Page::AA::WriteBackNoWriteAllocate,
        }
    }
}

impl CachePolicyEncoding<OuterEncoding, Page::Register> for CachePolicy {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            CachePolicy::NonCacheable => Page::BB::NonCacheable,
            CachePolicy::WriteBackWriteAllocate => Page::BB::WriteBackWriteAllocate,
            CachePolicy::WriteThroughNoWriteAllocate => Page::BB::WriteThroughNoWriteAllocate,
            CachePolicy::WriteBackNoWriteAllocate => Page::BB::WriteBackNoWriteAllocate,
        }
    }
}

impl FieldEncoding<Page::Register> for SharePolicy {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            SharePolicy::NonShareable => Page::S::NonShareable,
            SharePolicy::Shareable => Page::S::Shareable,
        }
    }
}

impl FieldEncoding<Page::Register> for MemoryAccess {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        self.read_write.encode() + self.execute.encode() + self.privilege.encode()
    }
}

impl FieldEncoding<Page::Register> for ReadWritePolicy {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            ReadWritePolicy::ReadOnly => Page::APX::DisableWrite,
            ReadWritePolicy::ReadWrite => Page::APX::EnableWrite,
        }
    }
}

impl FieldEncoding<Page::Register> for ExecutePolicy {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            ExecutePolicy::ExecuteNever => Page::XN::ExecuteNever,
            ExecutePolicy::Executeable => Page::XN::Executeable,
        }
    }
}

impl FieldEncoding<Page::Register> for PrivilegeLevel {
    fn encode(&self) -> FieldValue<u32, Page::Register> {
        match self {
            PrivilegeLevel::PrivilegeLevel0 => Page::AP::PL0,
            PrivilegeLevel::PrivilegeLevel1 => Page::AP::PL1,
        }
    }
}
