// In this file are all the helper functions used by the UI when decoding RigidModel PackedFiles.

use gtk::prelude::*;
use gtk::{
    Box, ScrolledWindow, Orientation, Button, Expander, Label, PolicyType, Entry
};
use packedfile::rigidmodel::RigidModelLodData;


/// Struct PackedFileRigidModelDataView: contains all the stuff we need to give to the program to
/// show a TreeView with the data of a RigidModel file, allowing us to manipulate it.
#[derive(Clone)]
pub struct PackedFileRigidModelDataView {
    pub packed_file_save_button: Button,
    pub rigid_model_game_label: Label,
    pub rigid_model_game_patch_button: Button,
    pub packed_file_texture_paths: Vec<Vec<Entry>>,
}

/// Implementation of "PackedFileRigidModelDataView"
impl PackedFileRigidModelDataView {

    /// This function creates a new Data View (custom layout) with "packed_file_data_display" as
    /// father and returns a PackedFileRigidModelDataView with all his data.
    pub fn create_data_view(
        packed_file_data_display: &Box,
        packed_file_decoded: &::packedfile::rigidmodel::RigidModel
    ) -> PackedFileRigidModelDataView {

        // Button for saving the PackedFile. It goes before everything, so it's not included in the
        // scrolledWindow.
        let packed_file_save_button = Button::new_with_label("Save to PackedFile");

        // Internal scrolledWindow, so if there are too many lods, we can scroll through them.
        // Inside it we put a box to fit all the labels and stuff properly.
        let packed_file_data_display_scroll = ScrolledWindow::new(None, None);
        let packed_file_data_display_scroll_inner_box = Box::new(Orientation::Vertical, 0);

        let rigid_model_game_box = Box::new(Orientation::Horizontal, 0);
        rigid_model_game_box.set_size_request(400, 0);

        let rigid_model_game_label = Label::new(Some(
            if packed_file_decoded.packed_file_header.packed_file_header_model_type == 6 {
                "RigidModel compatible with: \"Attila\"."
            }
            else {
                "RigidModel compatible with: \"Warhammer 1&2\"."
            }
        ));
        rigid_model_game_label.set_padding(4, 0);
        rigid_model_game_label.set_alignment(0.0, 0.5);

        let rigid_model_game_patch_button = Button::new_with_label("Patch to Warhammer 1&2");
        if packed_file_decoded.packed_file_header.packed_file_header_model_type == 6 {
            rigid_model_game_patch_button.set_sensitive(true);
        }
        else {
            rigid_model_game_patch_button.set_sensitive(false);
        }

        rigid_model_game_box.pack_start(&rigid_model_game_label, false, false, 0);
        rigid_model_game_box.pack_end(&rigid_model_game_patch_button, false, false, 0);

        // TODO: Improve this. Right now we get this info in a very unreliable way.
        let rigid_model_type_label = Label::new(Some(
            if !packed_file_decoded.packed_file_header.packed_file_data_base_skeleton.0.is_empty() {
                "RigidModel Type: \"Unit Model\"."
            }
            else if packed_file_decoded.packed_file_data.packed_file_data_lods_header[0].vertices_data_length == 0 {
                "RigidModel Type: \"Decal Model\"."
            }
            else {
                "RigidModel Type: \"Building/Prop Model\"."
        }));
        rigid_model_type_label.set_padding(4, 0);
        rigid_model_type_label.set_alignment(0.0, 0.5);

        let rigid_model_textures_label = Label::new(Some("Textures used by this RigidModel:"));
        rigid_model_textures_label.set_padding(4, 0);
        rigid_model_textures_label.set_alignment(0.0, 0.5);

        packed_file_data_display_scroll_inner_box.pack_start(&rigid_model_game_box, false, false, 0);
        packed_file_data_display_scroll_inner_box.pack_start(&rigid_model_type_label, false, false, 0);
        packed_file_data_display_scroll_inner_box.pack_start(&rigid_model_textures_label, false, false, 0);

        // TODO: Get this into his own function.
        // Here we process the Lods's texture paths.
        let mut packed_file_texture_paths = vec![];
        let mut index = 1;
        for lod in packed_file_decoded.packed_file_data.packed_file_data_lods_data.iter() {
            let lod_texture_expander = Expander::new(Some(&*format!("Lod {}", index)));
            let lod_texture_expander_box = Box::new(Orientation::Vertical, 0);
            lod_texture_expander.add(&lod_texture_expander_box);
            index += 1;

            let mut packed_file_texture_paths_lod = vec![];
            match lod.textures_list {
                Some(ref textures) => {

                    // If we have textures (not a decal) we decode their paths one by one
                    for texture in textures {

                        // First, we get it's type.
                        let texture_info_box = Box::new(Orientation::Horizontal, 0);
                        let texture_type = Label::new(Some(
                            //0 (Diffuse), 1 (Normal), 11 (Specular), 12 (Gloss), 3/10 (Mask), 5(no idea).
                            if texture.texture_type == 0 {
                                "Diffuse:"
                            }
                            else if texture.texture_type == 1 {
                                "Normal:"
                            }
                            else if texture.texture_type == 11 {
                                "Specular:"
                            }
                            else if texture.texture_type == 12 {
                                "Gloss:"
                            }
                            else if texture.texture_type == 3 || texture.texture_type == 10 {
                                "Mask:"
                            }
                            else if texture.texture_type == 5 {
                                "Unknown:"
                            }
                            else {
                                "Error. Unknown mask type."
                            }
                        ));

                        texture_type.set_alignment(0.0, 0.5);
                        texture_type.set_size_request(60, 0);

                        // Then we get it's path, and put it in a gtk::Entry.
                        let texture_path = Entry::new();
                        texture_path.get_buffer().set_text(&*texture.texture_path.0);
                        texture_path.get_buffer().set_max_length(Some(texture.texture_path.1 as u16));
                        texture_path.set_editable(true);

                        // We need to put a ScrolledWindow around the Entry, so we can move the
                        // text if it's too long.
                        let texture_path_scroll = ScrolledWindow::new(None, None);
                        texture_path_scroll.set_size_request(650, 0);
                        texture_path_scroll.set_policy(PolicyType::External, PolicyType::Never);
                        texture_path_scroll.set_max_content_width(600);
                        texture_path_scroll.add(&texture_path);

                        texture_info_box.pack_start(&texture_type, false, false, 10);
                        texture_info_box.pack_start(&texture_path_scroll, false, false, 0);
                        lod_texture_expander_box.pack_start(&texture_info_box, false, false, 0);

                        packed_file_texture_paths_lod.push(texture_path);
                    }
                }
                None => {
                    let texture_info_box = Box::new(Orientation::Horizontal, 0);
                    let texture_type = Label::new(Some("Texture Directory:"));
                    texture_type.set_alignment(0.0, 0.5);
                    texture_type.set_size_request(60, 0);

                    // Then we get it's path, and put it in a gtk::Entry.
                    let texture_path = Entry::new();
                    texture_path.get_buffer().set_text(&*lod.textures_directory.0);
                    texture_path.get_buffer().set_max_length(Some(lod.textures_directory.1 as u16));
                    texture_path.set_editable(true);

                    // We need to put a ScrolledWindow around the Entry, so we can move the
                    // text if it's too long.
                    let texture_path_scroll = ScrolledWindow::new(None, None);
                    texture_path_scroll.set_size_request(550, 0);
                    texture_path_scroll.set_policy(PolicyType::External, PolicyType::Never);
                    texture_path_scroll.set_max_content_width(500);
                    texture_path_scroll.add(&texture_path);

                    texture_info_box.pack_start(&texture_type, false, false, 10);
                    texture_info_box.pack_start(&texture_path_scroll, false, false, 0);
                    lod_texture_expander_box.pack_start(&texture_info_box, false, false, 0);

                    packed_file_texture_paths_lod.push(texture_path);
                }
            }
            packed_file_texture_paths.push(packed_file_texture_paths_lod);
            packed_file_data_display_scroll_inner_box.pack_start(&lod_texture_expander, false, false, 0);
        }

        packed_file_data_display_scroll.add(&packed_file_data_display_scroll_inner_box);
        packed_file_data_display.add(&packed_file_save_button);
        packed_file_data_display.pack_end(&packed_file_data_display_scroll, true, true, 0);
        packed_file_data_display.show_all();

        PackedFileRigidModelDataView {
            packed_file_save_button,
            rigid_model_game_label,
            rigid_model_game_patch_button,
            packed_file_texture_paths,
        }
    }

    /// This function get the texture path entries of a RigidModel from the UI and saves them into the
    /// opened RigidModel.
    pub fn return_data_from_data_view(
        packed_file_new_texture_paths: Vec<Vec<Entry>>,
        packed_file_data_lods_data: &mut Vec<RigidModelLodData>
    ) -> Vec<RigidModelLodData> {

        // If there is a texture list in the first lod, this is not a decal, so we try to get the
        // texture list of every lod.
        if let Some(_) = packed_file_data_lods_data[0].textures_list {
            for (index_lod, lod) in packed_file_new_texture_paths.iter().enumerate() {
                let mut texture_list = packed_file_data_lods_data[index_lod].clone().textures_list.unwrap();
                for (index_texture, texture) in lod.iter().enumerate() {
                    texture_list[index_texture].texture_path.0 = texture.get_text().unwrap();
                }
                packed_file_data_lods_data[index_lod].textures_list = Some(texture_list);


            }
        }

        // If there is no texture list, this is a decal. We just get the texture directory.
        else {
            for (index_lod, lod) in packed_file_new_texture_paths.iter().enumerate() {
                for (_, texture) in lod.iter().enumerate() {
                    packed_file_data_lods_data[index_lod].textures_directory.0 = texture.get_text().unwrap();
                }
            }
        }
        packed_file_data_lods_data.to_vec()
    }
}