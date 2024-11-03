use crate::commands::Command;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::builder::{CreateMessage, CreateSelectMenu, CreateSelectMenuOption, CreateActionRow};
use serenity::prelude::*;
use serenity::all::{ComponentInteractionDataKind, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, CreateSelectMenuKind};
use riven::consts::PlatformRoute;
use std::sync::Mutex;
use lazy_static::lazy_static;

fn debug_log(message: &str) {
    println!("[DEBUG] {}", message);
}

lazy_static! {
    static ref CURRENT_REGION: Mutex<PlatformRoute> = Mutex::new(PlatformRoute::RU); // Установите начальный регион по умолчанию
}

pub struct RegionCommand;

impl RegionCommand {
    pub fn new() -> Self {
        debug_log("Creating new RegionCommand instance.");
        RegionCommand
    }

    pub fn set_current_region(new_region: PlatformRoute) {
        let mut region = CURRENT_REGION.lock().unwrap();
        debug_log(&format!("Setting new current region: {:?}", new_region));
        *region = new_region;
    }
    
    pub fn get_region() -> PlatformRoute {
        let region = CURRENT_REGION.lock().unwrap();
        debug_log(&format!("Getting current region: {:?}", *region));
        *region
    }
    pub fn region_options(&self) -> Vec<CreateSelectMenuOption> {
        debug_log("Fetching region options.");
        vec![
            CreateSelectMenuOption::new("Brazil (BR1)", PlatformRoute::BR1.to_string()),
            CreateSelectMenuOption::new("EUNE (EUN1)", PlatformRoute::EUN1.to_string()),
            CreateSelectMenuOption::new("EUW (EUW1)", PlatformRoute::EUW1.to_string()),
            CreateSelectMenuOption::new("Japan (JP1)", PlatformRoute::JP1.to_string()),
            CreateSelectMenuOption::new("South Korea (KR)", PlatformRoute::KR.to_string()),
            CreateSelectMenuOption::new("North America (NA1)", PlatformRoute::NA1.to_string()),
            CreateSelectMenuOption::new("Oceania (OC1)", PlatformRoute::OC1.to_string()),
            CreateSelectMenuOption::new("Russia (RU)", PlatformRoute::RU.to_string()),
            CreateSelectMenuOption::new("Latin America (LA1)", PlatformRoute::LA1.to_string()),
            CreateSelectMenuOption::new("Latin America (LA2)", PlatformRoute::LA2.to_string()),
            CreateSelectMenuOption::new("Middle East (ME1)", PlatformRoute::ME1.to_string()),
            CreateSelectMenuOption::new("Philippines (PH2)", PlatformRoute::PH2.to_string()),
            CreateSelectMenuOption::new("Singapore (SG2)", PlatformRoute::SG2.to_string()),
            CreateSelectMenuOption::new("Thailand (TH2)", PlatformRoute::TH2.to_string()),
            CreateSelectMenuOption::new("Turkey (TR1)", PlatformRoute::TR1.to_string()),
            CreateSelectMenuOption::new("Taiwan (TW2)", PlatformRoute::TW2.to_string()),
            CreateSelectMenuOption::new("Vietnam (VN2)", PlatformRoute::VN2.to_string()),
            CreateSelectMenuOption::new("PBE (PBE1)", PlatformRoute::PBE1.to_string()),
        ]
    }

    pub fn get_region_string() -> String {
        let region = Self::get_region();
        debug_log(&format!("Converting platform {:?} to string.", region));
        match region {
            PlatformRoute::BR1 => "Brazil (BR1)".to_string(),
            PlatformRoute::EUN1 => "EUNE (EUN1)".to_string(),
            PlatformRoute::EUW1 => "EUW (EUW1)".to_string(),
            PlatformRoute::JP1 => "Japan (JP1)".to_string(),
            PlatformRoute::KR => "South Korea (KR)".to_string(),
            PlatformRoute::NA1 => "North America (NA1)".to_string(),
            PlatformRoute::OC1 => "Oceania (OC1)".to_string(),
            PlatformRoute::RU => "Russia (RU)".to_string(),
            PlatformRoute::LA1 => "Latin America (LA1)".to_string(),
            PlatformRoute::LA2 => "Latin America (LA2)".to_string(),
            PlatformRoute::ME1 => "Middle East (ME1)".to_string(),
            PlatformRoute::PH2 => "Philippines (PH2)".to_string(),
            PlatformRoute::SG2 => "Singapore (SG2)".to_string(),
            PlatformRoute::TH2 => "Thailand (TH2)".to_string(),
            PlatformRoute::TR1 => "Turkey (TR1)".to_string(),
            PlatformRoute::TW2 => "Taiwan (TW2)".to_string(),
            PlatformRoute::VN2 => "Vietnam (VN2)".to_string(),
            PlatformRoute::PBE1 => "PBE (PBE1)".to_string(),
            _ => "Unknown region".to_string(),
        }
    }
}


#[async_trait]
impl Command for RegionCommand {
    async fn handle(&self, ctx: &Context, msg: &Message, input: &str) {
        debug_log(&format!("Handling region command with input: '{}'", input));

      
        let new_region = match input {
            "BR1" => Some(PlatformRoute::BR1),
            "EUN1" => Some(PlatformRoute::EUN1),
            "EUW1" => Some(PlatformRoute::EUW1),
            "JP1" => Some(PlatformRoute::JP1),
            "KR" => Some(PlatformRoute::KR),
            "NA1" => Some(PlatformRoute::NA1),
            "OC1" => Some(PlatformRoute::OC1),
            "RU" => Some(PlatformRoute::RU),
            "LA1" => Some(PlatformRoute::LA1),
            "LA2" => Some(PlatformRoute::LA2),
            "ME1" => Some(PlatformRoute::ME1),
            "PH2" => Some(PlatformRoute::PH2),
            "SG2" => Some(PlatformRoute::SG2),
            "TH2" => Some(PlatformRoute::TH2),
            "TR1" => Some(PlatformRoute::TR1),
            "TW2" => Some(PlatformRoute::TW2),
            "VN2" => Some(PlatformRoute::VN2),
            "PBE1" => Some(PlatformRoute::PBE1),
            _ => None,
        };

        

        if let Some(region) = new_region {
            Self::set_current_region(region);
        } else {
            debug_log("No valid region provided, keeping current region.");
        }

        let current_region_str = Self::get_region_string();
        let response = format!("📍 **Current region:** `{}`", current_region_str);

        debug_log(&format!("Response message: {}", response));

        // Создаем меню выбора региона
        let select_menu = CreateSelectMenu::new(
            "region_select",
            CreateSelectMenuKind::String { options: self.region_options() }
        )
        .placeholder("Select a region");

        let action_row = CreateActionRow::SelectMenu(select_menu);

        let builder = CreateMessage::new()
            .content(response)
            .components(vec![action_row]);

        // Отправка сообщения
        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            println!("Error sending message: {:?}", why);
        } else {
            debug_log("Message sent successfully.");
        }
    }

    async fn handle_interaction(&self, ctx: &Context, interaction: &Interaction) {
        if let Some(component_interaction) = interaction.clone().message_component() {
            if let ComponentInteractionDataKind::StringSelect { values, .. } = &component_interaction.data.kind {
                if let Some(selected_value) = values.get(0) {
                    let new_region = match selected_value.as_str() {
                        "BR1" => Some(PlatformRoute::BR1),
                        "EUN1" => Some(PlatformRoute::EUN1),
                        "EUW1" => Some(PlatformRoute::EUW1),
                        "JP1" => Some(PlatformRoute::JP1),
                        "KR" => Some(PlatformRoute::KR),
                        "NA1" => Some(PlatformRoute::NA1),
                        "OC1" => Some(PlatformRoute::OC1),
                        "RU" => Some(PlatformRoute::RU),
                        "LA1" => Some(PlatformRoute::LA1),
                        "LA2" => Some(PlatformRoute::LA2),
                        "ME1" => Some(PlatformRoute::ME1),
                        "PH2" => Some(PlatformRoute::PH2),
                        "SG2" => Some(PlatformRoute::SG2),
                        "TH2" => Some(PlatformRoute::TH2),
                        "TR1" => Some(PlatformRoute::TR1),
                        "TW2" => Some(PlatformRoute::TW2),
                        "VN2" => Some(PlatformRoute::VN2),
                        "PBE1" => Some(PlatformRoute::PBE1),
                        _ => None,
                    };

                    

                    if let Some(region) = new_region {
                        RegionCommand::set_current_region(region);
                    }

                    let response_message = format!(
                        "📍 **Current region:** `{}`",
                        RegionCommand::get_region_string()
                    );

                    let create_response = CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::default().content(&response_message)
                    );

                    if let Err(why) = component_interaction.create_response(&ctx.http, create_response).await {
                        debug_log(&format!("Error responding to interaction: {:?}", why));
                    }

                    return;
                }
            }
        }

        debug_log("Interaction not handled or no value selected.");
    }

    fn name(&self) -> &str {
        "region"
    }

    fn description(&self) -> &str {
        "Show the current region and select a new one."
    }
}