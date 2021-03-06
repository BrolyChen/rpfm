//---------------------------------------------------------------------------//
// Copyright (c) 2017-2020 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module with all the code related to the main `DiagnosticsUISlots`.
!*/

use qt_core::QBox;
use qt_core::{SlotNoArgs, SlotOfBool, SlotOfQModelIndex};

use std::rc::Rc;

use crate::AppUI;
use crate::diagnostics_ui::DiagnosticsUI;
use crate::global_search_ui::GlobalSearchUI;
use crate::packfile_contents_ui::PackFileContentsUI;

//-------------------------------------------------------------------------------//
//                              Enums & Structs
//-------------------------------------------------------------------------------//

/// This struct contains all the slots we need to respond to signals of the diagnostics panel.
pub struct DiagnosticsUISlots {
    pub diagnostics_open_result: QBox<SlotOfQModelIndex>,
    pub show_hide_extra_filters: QBox<SlotOfBool>,
    pub toggle_filters: QBox<SlotNoArgs>,
    pub toggle_filters_types: QBox<SlotNoArgs>,
}

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

/// Implementation of `DiagnosticsUISlots`.
impl DiagnosticsUISlots {

    /// This function creates an entire `DiagnosticsUISlots` struct.
    pub unsafe fn new(
        app_ui: &Rc<AppUI>,
        pack_file_contents_ui: &Rc<PackFileContentsUI>,
        global_search_ui: &Rc<GlobalSearchUI>,
        diagnostics_ui: &Rc<DiagnosticsUI>,
    ) -> Self {

        // What happens when we try to open the file corresponding to one of the matches.
        let diagnostics_open_result = SlotOfQModelIndex::new(&diagnostics_ui.diagnostics_dock_widget, clone!(
            app_ui,
            pack_file_contents_ui,
            global_search_ui,
            diagnostics_ui => move |model_index_filter| {
                DiagnosticsUI::open_match(&app_ui, &pack_file_contents_ui, &global_search_ui, &diagnostics_ui, model_index_filter.as_ptr());
            }
        ));

        let show_hide_extra_filters = SlotOfBool::new(&diagnostics_ui.diagnostics_dock_widget, clone!(
            diagnostics_ui => move |state| {
                if !state { diagnostics_ui.sidebar_scroll_area.hide(); }
                else { diagnostics_ui.sidebar_scroll_area.show();}
            }
        ));

        let toggle_filters = SlotNoArgs::new(&diagnostics_ui.diagnostics_dock_widget, clone!(
            app_ui,
            diagnostics_ui => move || {
            DiagnosticsUI::filter(&app_ui, &diagnostics_ui);
        }));

        let toggle_filters_types = SlotNoArgs::new(&diagnostics_ui.diagnostics_dock_widget, clone!(
            diagnostics_ui => move || {
                diagnostics_ui.checkbox_outdated_table.toggle();
                diagnostics_ui.checkbox_invalid_reference.toggle();
                diagnostics_ui.checkbox_empty_row.toggle();
                diagnostics_ui.checkbox_empty_key_field.toggle();
                diagnostics_ui.checkbox_empty_key_fields.toggle();
                diagnostics_ui.checkbox_duplicated_combined_keys.toggle();
                diagnostics_ui.checkbox_no_reference_table_found.toggle();
                diagnostics_ui.checkbox_no_reference_table_nor_column_found_pak.toggle();
                diagnostics_ui.checkbox_no_reference_table_nor_column_found_no_pak.toggle();
                diagnostics_ui.checkbox_invalid_escape.toggle();
                diagnostics_ui.checkbox_duplicated_row.toggle();
                diagnostics_ui.checkbox_invalid_dependency_packfile.toggle();
            }
        ));

        // And here... we return all the slots.
        Self {
            diagnostics_open_result,
            show_hide_extra_filters,
            toggle_filters,
            toggle_filters_types,
        }
    }
}
