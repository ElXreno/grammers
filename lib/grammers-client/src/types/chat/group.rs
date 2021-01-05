// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use grammers_tl_types as tl;
use std::fmt;

/// A group chat.
///
/// Telegram's API internally distinguishes between "small group chats" and "megagroups", also
/// known as "supergroups" in the UI of Telegram applications.
///
/// Small group chats are the default, and offer less features than megagroups, but you can
/// join more of them. Certain actions in official clients, like setting a chat's username,
/// silently upgrade the chat to a megagroup.
#[derive(Clone)]
pub struct Group(tl::enums::Chat);

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// TODO it might be desirable to manually merge all the properties of the chat to avoid endless matching

impl Group {
    pub(crate) fn from_raw(_chat: tl::enums::Chat) -> Self {
        todo!()
    }

    pub(crate) fn to_peer(&self) -> tl::enums::Peer {
        use tl::enums::Chat;

        match &self.0 {
            Chat::Empty(chat) => tl::types::PeerChat { chat_id: chat.id }.into(),
            Chat::Chat(chat) => tl::types::PeerChat { chat_id: chat.id }.into(),
            Chat::Forbidden(chat) => tl::types::PeerChat { chat_id: chat.id }.into(),
            Chat::Channel(chat) => tl::types::PeerChannel {
                channel_id: chat.id,
            }
            .into(),
            Chat::ChannelForbidden(chat) => tl::types::PeerChannel {
                channel_id: chat.id,
            }
            .into(),
        }
    }

    pub(crate) fn to_input_peer(&self) -> tl::enums::InputPeer {
        todo!()
    }

    pub(crate) fn to_input_channel(&self) -> Option<tl::enums::InputChannel> {
        todo!()
    }

    pub(crate) fn to_chat_id(&self) -> Option<i32> {
        todo!()
    }

    /// Return the unique identifier for this group.
    ///
    /// Note that if this group is migrated to a megagroup, both this group and the new one will
    /// exist as separate chats, with different identifiers.
    pub fn id(&self) -> i32 {
        use tl::enums::Chat;

        match &self.0 {
            Chat::Empty(chat) => chat.id,
            Chat::Chat(chat) => chat.id,
            Chat::Forbidden(chat) => chat.id,
            Chat::Channel(chat) => chat.id,
            Chat::ChannelForbidden(chat) => chat.id,
        }
    }

    /// Return the title of this group.
    ///
    /// The title may be the empty string if the group is not accessible.
    pub fn title(&self) -> &str {
        use tl::enums::Chat;

        match &self.0 {
            Chat::Empty(_) => "",
            Chat::Chat(chat) => chat.title.as_str(),
            Chat::Forbidden(chat) => chat.title.as_str(),
            Chat::Channel(chat) => chat.title.as_str(),
            Chat::ChannelForbidden(chat) => chat.title.as_str(),
        }
    }
}