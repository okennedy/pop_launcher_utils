
// Borrowed from the PopOS Launcher documentation
// https://github.com/pop-os/launcher
// And adapted for serde_json
// https://github.com/serde-rs/json

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type Generation = u32;
pub type Indice = u32;

#[derive(Serialize, Deserialize, Clone)]
pub enum Request {
    /// Activate on the selected item
    Activate(Indice),
    /// Activate a context item on an item.
    ActivateContext { id: Indice, context: Indice },
    /// Perform a tab completion from the selected item
    Complete(Indice),
    /// Request for any context options this result may have.
    Context(Indice),
    /// Request to end the service
    Exit,
    /// Requests to cancel any active searches
    Interrupt,
    /// Request to close the selected item
    Quit(Indice),
    /// Perform a search in our database
    Search(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum IconSource {
    // Locate by name or path.
    Name(String),
    // Icon is a mime type.
    Mime(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginSearchResult {
    /// Numeric identifier tracked by the plugin.
    pub id: Indice,
    /// The name / title.
    pub name: String,
    /// The description / subtitle.
    pub description: String,
    /// Extra words to match when sorting and filtering.
    pub keywords: Option<Vec<String>>,
    /// Icon to display in the frontend.
    pub icon: Option<IconSource>,
    /// Command that is executed by this result, used for sorting and filtering.
    pub exec: Option<String>,
    /// Designates that this search item refers to a window.
    pub window: Option<(Generation, Indice)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContextOption {
    pub id: Indice,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GpuPreference {
    Default,
    NonDefault,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PluginResponse {
    /// Append a new search item to the launcher
    Append(PluginSearchResult),
    /// Clear all results in the launcher list
    Clear,
    /// Close the launcher
    Close,
    // Additional options for launching a certain item
    Context {
        id: Indice,
        options: Vec<ContextOption>,
    },
    // Notifies that a .desktop entry should be launched by the frontend.
    DesktopEntry {
        path: PathBuf,
        gpu_preference: GpuPreference,
    },
    /// Update the text in the launcher
    Fill(String),
    /// Indicates that a plugin is finished with its queries
    Finished,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResult {
    /// Numeric identifier tracked by the plugin.
    pub id: Indice,
    /// The name / title.
    pub name: String,
    /// The description / subtitle.
    pub description: String,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    /// Icon to display in the frontend for this item
    pub icon: Option<IconSource>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    /// Icon to display in the frontend for this plugin
    pub category_icon: Option<IconSource>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::unwrap_or_skip"
    )]
    /// Designates that this search item refers to a window.
    pub window: Option<(Generation, Indice)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Response {
    // An operation was performed and the frontend may choose to exit its process.
    Close,
    // Additional options for launching a certain item
    Context {
        id: Indice,
        options: Vec<ContextOption>,
    },
    // Notifies that a .desktop entry should be launched by the frontend.
    DesktopEntry {
        path: PathBuf,
        gpu_preference: GpuPreference,
    },
    // The frontend should clear its search results and display a new list
    Update(Vec<SearchResult>),
    // An item was selected that resulted in a need to autofill the launcher
    Fill(String),
}

