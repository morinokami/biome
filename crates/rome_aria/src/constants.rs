//! Metadata of:
//! - ARIA properties
//! - ARIA property types
//! - ARIA roles

pub const ARIA_PROPERTIES: [&str; 48] = [
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-checked",
    "aria-colcount",
    "aria-colindex",
    "aria-colspan",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-expanded",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-modal",
    "aria-multiline",
    "aria-multiselectable",
    "aria-orientation",
    "aria-owns",
    "aria-placeholder",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-roledescription",
    "aria-rowcount",
    "aria-rowindex",
    "aria-rowspan",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "aria-valuetext",
];

pub const ARIA_PROPERTY_TYPE: [&str; 9] = [
    "boolean",
    "id",
    "idlist",
    "integer",
    "number",
    "string",
    "token",
    "tokenlist",
    "tristate",
];

pub const ARIA_WIDGET_ROLES: [&str; 27] = [
    "alert",
    "alertdialog",
    "button",
    "checkbox",
    "dialog",
    "gridcell",
    "link",
    "log",
    "marquee",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "option",
    "progressbar",
    "radio",
    "scrollbar",
    "searchbox",
    "slider",
    "spinbutton",
    "status",
    "switch",
    "tab",
    "tabpanel",
    "textbox",
    "timer",
    "tooltip",
    "treeitem",
];

pub const ARIA_ABSTRACT_ROLES: [&str; 12] = [
    "command",
    "composite",
    "input",
    "landmark",
    "range",
    "roletype",
    "section",
    "sectionhead",
    "select",
    "structure",
    "widget",
    "window",
];

pub const ARIA_DOCUMENT_STRUCTURE_ROLES: [&str; 25] = [
    "article",
    "cell",
    "columnheader",
    "definition",
    "directory",
    "document",
    "feed",
    "figure",
    "group",
    "heading",
    "img",
    "list",
    "listitem",
    "math",
    "none",
    "note",
    "presentation",
    "region",
    "row",
    "rowgroup",
    "rowheader",
    "separator",
    "table",
    "term",
    "toolbar",
];