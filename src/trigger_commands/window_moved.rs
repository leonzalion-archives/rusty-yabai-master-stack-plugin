use crate::context::{
	window::{create_windows_manager, UpdateWindowsProps},
	YabaiPlugin,
};

pub fn window_moved(plugin: &YabaiPlugin) {
	log::debug!("Starting to handle window_moved.");
	let mut wm = create_windows_manager(plugin);
	let state = plugin.read_state();
	wm.update_windows(UpdateWindowsProps {
		target_num_master_windows: state.num_master_windows[&wm.space.id],
	});
	log::debug!("Finished handling window_moved.");
}
