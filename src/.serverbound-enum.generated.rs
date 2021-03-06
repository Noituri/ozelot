/* This file is automatically generated by packets.clj
Do not manually edit this file, if you wish to make
changes here, then edit and rerun packets.clj */

/// Represents a single packet
#[derive(Debug, PartialEq, Clone)]
pub enum ServerboundPacket {
    Handshake(Handshake),
    StatusRequest(StatusRequest),
    StatusPing(StatusPing),
    LoginStart(LoginStart),
    EncryptionResponse(EncryptionResponse),
    LoginPluginResponse(LoginPluginResponse),
    TeleportConfirm(TeleportConfirm),
    QueryBlockNBT(QueryBlockNBT),
    SetDifficulty(SetDifficulty),
    ChatMessage(ChatMessage),
    ClientStatus(ClientStatus),
    ClientSettings(ClientSettings),
    TabComplete(TabComplete),
    ConfirmTransaction(ConfirmTransaction),
    EnchantItem(EnchantItem),
    ClickWindow(ClickWindow),
    CloseWindow(CloseWindow),
    PluginMessage(PluginMessage),
    EditBook(EditBook),
    QueryEntityNBT(QueryEntityNBT),
    UseEntity(UseEntity),
    KeepAlive(KeepAlive),
    LockDifficulty(LockDifficulty),
    PlayerPosition(PlayerPosition),
    PlayerPositionAndLook(PlayerPositionAndLook),
    PlayerLook(PlayerLook),
    Player(Player),
    VehicleMove(VehicleMove),
    SteerBoat(SteerBoat),
    PickItem(PickItem),
    CraftRecipeRequest(CraftRecipeRequest),
    PlayerAbilities(PlayerAbilities),
    PlayerDigging(PlayerDigging),
    EntityAction(EntityAction),
    SteerVehicle(SteerVehicle),
    RecipeBookData(RecipeBookData),
    NameItem(NameItem),
    ResourcePackStatus(ResourcePackStatus),
    AdvancementTab(AdvancementTab),
    SelectTrade(SelectTrade),
    SetBeaconEffect(SetBeaconEffect),
    HeldItemChange(HeldItemChange),
    UpdateCommandBlock(UpdateCommandBlock),
    UpdateCommandBlockMinecart(UpdateCommandBlockMinecart),
    CreativeInventoryAction(CreativeInventoryAction),
    UpdateJigsawBlock(UpdateJigsawBlock),
    UpdateStructureBlock(UpdateStructureBlock),
    UpdateSign(UpdateSign),
    Animation(Animation),
    Spectate(Spectate),
    PlayerBlockPlacement(PlayerBlockPlacement),
    UseItem(UseItem),

}

impl Packet for ServerboundPacket {
    fn deserialize<R: Read>(r: &mut R, state: &ClientState) -> Result<Self> {
        let packet_id = read_varint(r)?;
        match state {
        &ClientState::Handshake => {
            match packet_id {
            0 => Ok(Handshake::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Status => {
            match packet_id {
            0 => Ok(StatusRequest::deserialize(r)?),
            1 => Ok(StatusPing::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Login => {
            match packet_id {
            0 => Ok(LoginStart::deserialize(r)?),
            1 => Ok(EncryptionResponse::deserialize(r)?),
            2 => Ok(LoginPluginResponse::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },
        &ClientState::Play => {
            match packet_id {
            0 => Ok(TeleportConfirm::deserialize(r)?),
            1 => Ok(QueryBlockNBT::deserialize(r)?),
            2 => Ok(SetDifficulty::deserialize(r)?),
            3 => Ok(ChatMessage::deserialize(r)?),
            4 => Ok(ClientStatus::deserialize(r)?),
            5 => Ok(ClientSettings::deserialize(r)?),
            6 => Ok(TabComplete::deserialize(r)?),
            7 => Ok(ConfirmTransaction::deserialize(r)?),
            8 => Ok(EnchantItem::deserialize(r)?),
            9 => Ok(ClickWindow::deserialize(r)?),
            10 => Ok(CloseWindow::deserialize(r)?),
            11 => Ok(PluginMessage::deserialize(r)?),
            12 => Ok(EditBook::deserialize(r)?),
            13 => Ok(QueryEntityNBT::deserialize(r)?),
            14 => Ok(UseEntity::deserialize(r)?),
            15 => Ok(KeepAlive::deserialize(r)?),
            16 => Ok(LockDifficulty::deserialize(r)?),
            17 => Ok(PlayerPosition::deserialize(r)?),
            18 => Ok(PlayerPositionAndLook::deserialize(r)?),
            19 => Ok(PlayerLook::deserialize(r)?),
            20 => Ok(Player::deserialize(r)?),
            21 => Ok(VehicleMove::deserialize(r)?),
            22 => Ok(SteerBoat::deserialize(r)?),
            23 => Ok(PickItem::deserialize(r)?),
            24 => Ok(CraftRecipeRequest::deserialize(r)?),
            25 => Ok(PlayerAbilities::deserialize(r)?),
            26 => Ok(PlayerDigging::deserialize(r)?),
            27 => Ok(EntityAction::deserialize(r)?),
            28 => Ok(SteerVehicle::deserialize(r)?),
            29 => Ok(RecipeBookData::deserialize(r)?),
            30 => Ok(NameItem::deserialize(r)?),
            31 => Ok(ResourcePackStatus::deserialize(r)?),
            32 => Ok(AdvancementTab::deserialize(r)?),
            33 => Ok(SelectTrade::deserialize(r)?),
            34 => Ok(SetBeaconEffect::deserialize(r)?),
            35 => Ok(HeldItemChange::deserialize(r)?),
            36 => Ok(UpdateCommandBlock::deserialize(r)?),
            37 => Ok(UpdateCommandBlockMinecart::deserialize(r)?),
            38 => Ok(CreativeInventoryAction::deserialize(r)?),
            39 => Ok(UpdateJigsawBlock::deserialize(r)?),
            40 => Ok(UpdateStructureBlock::deserialize(r)?),
            41 => Ok(UpdateSign::deserialize(r)?),
            42 => Ok(Animation::deserialize(r)?),
            43 => Ok(Spectate::deserialize(r)?),
            44 => Ok(PlayerBlockPlacement::deserialize(r)?),
            45 => Ok(UseItem::deserialize(r)?),

            _ => bail!("No packet with id {} in state {}", packet_id, state),
            }
        },

        }
    }
    fn get_packet_name(&self) -> &str {
        match self {
        &ServerboundPacket::Handshake(..) => "Handshake",
        &ServerboundPacket::StatusRequest(..) => "StatusRequest",
        &ServerboundPacket::StatusPing(..) => "StatusPing",
        &ServerboundPacket::LoginStart(..) => "LoginStart",
        &ServerboundPacket::EncryptionResponse(..) => "EncryptionResponse",
        &ServerboundPacket::LoginPluginResponse(..) => "LoginPluginResponse",
        &ServerboundPacket::TeleportConfirm(..) => "TeleportConfirm",
        &ServerboundPacket::QueryBlockNBT(..) => "QueryBlockNBT",
        &ServerboundPacket::SetDifficulty(..) => "SetDifficulty",
        &ServerboundPacket::ChatMessage(..) => "ChatMessage",
        &ServerboundPacket::ClientStatus(..) => "ClientStatus",
        &ServerboundPacket::ClientSettings(..) => "ClientSettings",
        &ServerboundPacket::TabComplete(..) => "TabComplete",
        &ServerboundPacket::ConfirmTransaction(..) => "ConfirmTransaction",
        &ServerboundPacket::EnchantItem(..) => "EnchantItem",
        &ServerboundPacket::ClickWindow(..) => "ClickWindow",
        &ServerboundPacket::CloseWindow(..) => "CloseWindow",
        &ServerboundPacket::PluginMessage(..) => "PluginMessage",
        &ServerboundPacket::EditBook(..) => "EditBook",
        &ServerboundPacket::QueryEntityNBT(..) => "QueryEntityNBT",
        &ServerboundPacket::UseEntity(..) => "UseEntity",
        &ServerboundPacket::KeepAlive(..) => "KeepAlive",
        &ServerboundPacket::LockDifficulty(..) => "LockDifficulty",
        &ServerboundPacket::PlayerPosition(..) => "PlayerPosition",
        &ServerboundPacket::PlayerPositionAndLook(..) => "PlayerPositionAndLook",
        &ServerboundPacket::PlayerLook(..) => "PlayerLook",
        &ServerboundPacket::Player(..) => "Player",
        &ServerboundPacket::VehicleMove(..) => "VehicleMove",
        &ServerboundPacket::SteerBoat(..) => "SteerBoat",
        &ServerboundPacket::PickItem(..) => "PickItem",
        &ServerboundPacket::CraftRecipeRequest(..) => "CraftRecipeRequest",
        &ServerboundPacket::PlayerAbilities(..) => "PlayerAbilities",
        &ServerboundPacket::PlayerDigging(..) => "PlayerDigging",
        &ServerboundPacket::EntityAction(..) => "EntityAction",
        &ServerboundPacket::SteerVehicle(..) => "SteerVehicle",
        &ServerboundPacket::RecipeBookData(..) => "RecipeBookData",
        &ServerboundPacket::NameItem(..) => "NameItem",
        &ServerboundPacket::ResourcePackStatus(..) => "ResourcePackStatus",
        &ServerboundPacket::AdvancementTab(..) => "AdvancementTab",
        &ServerboundPacket::SelectTrade(..) => "SelectTrade",
        &ServerboundPacket::SetBeaconEffect(..) => "SetBeaconEffect",
        &ServerboundPacket::HeldItemChange(..) => "HeldItemChange",
        &ServerboundPacket::UpdateCommandBlock(..) => "UpdateCommandBlock",
        &ServerboundPacket::UpdateCommandBlockMinecart(..) => "UpdateCommandBlockMinecart",
        &ServerboundPacket::CreativeInventoryAction(..) => "CreativeInventoryAction",
        &ServerboundPacket::UpdateJigsawBlock(..) => "UpdateJigsawBlock",
        &ServerboundPacket::UpdateStructureBlock(..) => "UpdateStructureBlock",
        &ServerboundPacket::UpdateSign(..) => "UpdateSign",
        &ServerboundPacket::Animation(..) => "Animation",
        &ServerboundPacket::Spectate(..) => "Spectate",
        &ServerboundPacket::PlayerBlockPlacement(..) => "PlayerBlockPlacement",
        &ServerboundPacket::UseItem(..) => "UseItem",

        }
    }
    fn get_clientstate(&self) -> ClientState {
        match self {
        &ServerboundPacket::Handshake(..) => ClientState::Handshake,
        &ServerboundPacket::StatusRequest(..) => ClientState::Status,
        &ServerboundPacket::StatusPing(..) => ClientState::Status,
        &ServerboundPacket::LoginStart(..) => ClientState::Login,
        &ServerboundPacket::EncryptionResponse(..) => ClientState::Login,
        &ServerboundPacket::LoginPluginResponse(..) => ClientState::Login,
        &ServerboundPacket::TeleportConfirm(..) => ClientState::Play,
        &ServerboundPacket::QueryBlockNBT(..) => ClientState::Play,
        &ServerboundPacket::SetDifficulty(..) => ClientState::Play,
        &ServerboundPacket::ChatMessage(..) => ClientState::Play,
        &ServerboundPacket::ClientStatus(..) => ClientState::Play,
        &ServerboundPacket::ClientSettings(..) => ClientState::Play,
        &ServerboundPacket::TabComplete(..) => ClientState::Play,
        &ServerboundPacket::ConfirmTransaction(..) => ClientState::Play,
        &ServerboundPacket::EnchantItem(..) => ClientState::Play,
        &ServerboundPacket::ClickWindow(..) => ClientState::Play,
        &ServerboundPacket::CloseWindow(..) => ClientState::Play,
        &ServerboundPacket::PluginMessage(..) => ClientState::Play,
        &ServerboundPacket::EditBook(..) => ClientState::Play,
        &ServerboundPacket::QueryEntityNBT(..) => ClientState::Play,
        &ServerboundPacket::UseEntity(..) => ClientState::Play,
        &ServerboundPacket::KeepAlive(..) => ClientState::Play,
        &ServerboundPacket::LockDifficulty(..) => ClientState::Play,
        &ServerboundPacket::PlayerPosition(..) => ClientState::Play,
        &ServerboundPacket::PlayerPositionAndLook(..) => ClientState::Play,
        &ServerboundPacket::PlayerLook(..) => ClientState::Play,
        &ServerboundPacket::Player(..) => ClientState::Play,
        &ServerboundPacket::VehicleMove(..) => ClientState::Play,
        &ServerboundPacket::SteerBoat(..) => ClientState::Play,
        &ServerboundPacket::PickItem(..) => ClientState::Play,
        &ServerboundPacket::CraftRecipeRequest(..) => ClientState::Play,
        &ServerboundPacket::PlayerAbilities(..) => ClientState::Play,
        &ServerboundPacket::PlayerDigging(..) => ClientState::Play,
        &ServerboundPacket::EntityAction(..) => ClientState::Play,
        &ServerboundPacket::SteerVehicle(..) => ClientState::Play,
        &ServerboundPacket::RecipeBookData(..) => ClientState::Play,
        &ServerboundPacket::NameItem(..) => ClientState::Play,
        &ServerboundPacket::ResourcePackStatus(..) => ClientState::Play,
        &ServerboundPacket::AdvancementTab(..) => ClientState::Play,
        &ServerboundPacket::SelectTrade(..) => ClientState::Play,
        &ServerboundPacket::SetBeaconEffect(..) => ClientState::Play,
        &ServerboundPacket::HeldItemChange(..) => ClientState::Play,
        &ServerboundPacket::UpdateCommandBlock(..) => ClientState::Play,
        &ServerboundPacket::UpdateCommandBlockMinecart(..) => ClientState::Play,
        &ServerboundPacket::CreativeInventoryAction(..) => ClientState::Play,
        &ServerboundPacket::UpdateJigsawBlock(..) => ClientState::Play,
        &ServerboundPacket::UpdateStructureBlock(..) => ClientState::Play,
        &ServerboundPacket::UpdateSign(..) => ClientState::Play,
        &ServerboundPacket::Animation(..) => ClientState::Play,
        &ServerboundPacket::Spectate(..) => ClientState::Play,
        &ServerboundPacket::PlayerBlockPlacement(..) => ClientState::Play,
        &ServerboundPacket::UseItem(..) => ClientState::Play,

        }
    }
    fn get_id(&self) -> i32 {
        match self {
        &ServerboundPacket::Handshake(..) => 0,
        &ServerboundPacket::StatusRequest(..) => 0,
        &ServerboundPacket::StatusPing(..) => 1,
        &ServerboundPacket::LoginStart(..) => 0,
        &ServerboundPacket::EncryptionResponse(..) => 1,
        &ServerboundPacket::LoginPluginResponse(..) => 2,
        &ServerboundPacket::TeleportConfirm(..) => 0,
        &ServerboundPacket::QueryBlockNBT(..) => 1,
        &ServerboundPacket::SetDifficulty(..) => 2,
        &ServerboundPacket::ChatMessage(..) => 3,
        &ServerboundPacket::ClientStatus(..) => 4,
        &ServerboundPacket::ClientSettings(..) => 5,
        &ServerboundPacket::TabComplete(..) => 6,
        &ServerboundPacket::ConfirmTransaction(..) => 7,
        &ServerboundPacket::EnchantItem(..) => 8,
        &ServerboundPacket::ClickWindow(..) => 9,
        &ServerboundPacket::CloseWindow(..) => 10,
        &ServerboundPacket::PluginMessage(..) => 11,
        &ServerboundPacket::EditBook(..) => 12,
        &ServerboundPacket::QueryEntityNBT(..) => 13,
        &ServerboundPacket::UseEntity(..) => 14,
        &ServerboundPacket::KeepAlive(..) => 15,
        &ServerboundPacket::LockDifficulty(..) => 16,
        &ServerboundPacket::PlayerPosition(..) => 17,
        &ServerboundPacket::PlayerPositionAndLook(..) => 18,
        &ServerboundPacket::PlayerLook(..) => 19,
        &ServerboundPacket::Player(..) => 20,
        &ServerboundPacket::VehicleMove(..) => 21,
        &ServerboundPacket::SteerBoat(..) => 22,
        &ServerboundPacket::PickItem(..) => 23,
        &ServerboundPacket::CraftRecipeRequest(..) => 24,
        &ServerboundPacket::PlayerAbilities(..) => 25,
        &ServerboundPacket::PlayerDigging(..) => 26,
        &ServerboundPacket::EntityAction(..) => 27,
        &ServerboundPacket::SteerVehicle(..) => 28,
        &ServerboundPacket::RecipeBookData(..) => 29,
        &ServerboundPacket::NameItem(..) => 30,
        &ServerboundPacket::ResourcePackStatus(..) => 31,
        &ServerboundPacket::AdvancementTab(..) => 32,
        &ServerboundPacket::SelectTrade(..) => 33,
        &ServerboundPacket::SetBeaconEffect(..) => 34,
        &ServerboundPacket::HeldItemChange(..) => 35,
        &ServerboundPacket::UpdateCommandBlock(..) => 36,
        &ServerboundPacket::UpdateCommandBlockMinecart(..) => 37,
        &ServerboundPacket::CreativeInventoryAction(..) => 38,
        &ServerboundPacket::UpdateJigsawBlock(..) => 39,
        &ServerboundPacket::UpdateStructureBlock(..) => 40,
        &ServerboundPacket::UpdateSign(..) => 41,
        &ServerboundPacket::Animation(..) => 42,
        &ServerboundPacket::Spectate(..) => 43,
        &ServerboundPacket::PlayerBlockPlacement(..) => 44,
        &ServerboundPacket::UseItem(..) => 45,

        }
    }
    fn to_u8(&self) -> Result<Vec<u8>> {
        match self {
        &ServerboundPacket::Handshake(ref x) => x.to_u8(),
        &ServerboundPacket::StatusRequest(ref x) => x.to_u8(),
        &ServerboundPacket::StatusPing(ref x) => x.to_u8(),
        &ServerboundPacket::LoginStart(ref x) => x.to_u8(),
        &ServerboundPacket::EncryptionResponse(ref x) => x.to_u8(),
        &ServerboundPacket::LoginPluginResponse(ref x) => x.to_u8(),
        &ServerboundPacket::TeleportConfirm(ref x) => x.to_u8(),
        &ServerboundPacket::QueryBlockNBT(ref x) => x.to_u8(),
        &ServerboundPacket::SetDifficulty(ref x) => x.to_u8(),
        &ServerboundPacket::ChatMessage(ref x) => x.to_u8(),
        &ServerboundPacket::ClientStatus(ref x) => x.to_u8(),
        &ServerboundPacket::ClientSettings(ref x) => x.to_u8(),
        &ServerboundPacket::TabComplete(ref x) => x.to_u8(),
        &ServerboundPacket::ConfirmTransaction(ref x) => x.to_u8(),
        &ServerboundPacket::EnchantItem(ref x) => x.to_u8(),
        &ServerboundPacket::ClickWindow(ref x) => x.to_u8(),
        &ServerboundPacket::CloseWindow(ref x) => x.to_u8(),
        &ServerboundPacket::PluginMessage(ref x) => x.to_u8(),
        &ServerboundPacket::EditBook(ref x) => x.to_u8(),
        &ServerboundPacket::QueryEntityNBT(ref x) => x.to_u8(),
        &ServerboundPacket::UseEntity(ref x) => x.to_u8(),
        &ServerboundPacket::KeepAlive(ref x) => x.to_u8(),
        &ServerboundPacket::LockDifficulty(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerPosition(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerPositionAndLook(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerLook(ref x) => x.to_u8(),
        &ServerboundPacket::Player(ref x) => x.to_u8(),
        &ServerboundPacket::VehicleMove(ref x) => x.to_u8(),
        &ServerboundPacket::SteerBoat(ref x) => x.to_u8(),
        &ServerboundPacket::PickItem(ref x) => x.to_u8(),
        &ServerboundPacket::CraftRecipeRequest(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerAbilities(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerDigging(ref x) => x.to_u8(),
        &ServerboundPacket::EntityAction(ref x) => x.to_u8(),
        &ServerboundPacket::SteerVehicle(ref x) => x.to_u8(),
        &ServerboundPacket::RecipeBookData(ref x) => x.to_u8(),
        &ServerboundPacket::NameItem(ref x) => x.to_u8(),
        &ServerboundPacket::ResourcePackStatus(ref x) => x.to_u8(),
        &ServerboundPacket::AdvancementTab(ref x) => x.to_u8(),
        &ServerboundPacket::SelectTrade(ref x) => x.to_u8(),
        &ServerboundPacket::SetBeaconEffect(ref x) => x.to_u8(),
        &ServerboundPacket::HeldItemChange(ref x) => x.to_u8(),
        &ServerboundPacket::UpdateCommandBlock(ref x) => x.to_u8(),
        &ServerboundPacket::UpdateCommandBlockMinecart(ref x) => x.to_u8(),
        &ServerboundPacket::CreativeInventoryAction(ref x) => x.to_u8(),
        &ServerboundPacket::UpdateJigsawBlock(ref x) => x.to_u8(),
        &ServerboundPacket::UpdateStructureBlock(ref x) => x.to_u8(),
        &ServerboundPacket::UpdateSign(ref x) => x.to_u8(),
        &ServerboundPacket::Animation(ref x) => x.to_u8(),
        &ServerboundPacket::Spectate(ref x) => x.to_u8(),
        &ServerboundPacket::PlayerBlockPlacement(ref x) => x.to_u8(),
        &ServerboundPacket::UseItem(ref x) => x.to_u8(),

        }
    }
}
impl fmt::Display for ServerboundPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServerboundPacket of type {}", self.get_packet_name())
    }
}