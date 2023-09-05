use std::env::var;
use std::io::Read;
use std::time::Duration;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, Error, MimeType, Seat};
use wl_clipboard_rs::utils::is_primary_selection_supported;
use x11_clipboard::Clipboard;

// TODO
pub fn get_text() -> String {
	// match var("XDG_SESSION_TYPE") {
	// 	Ok(session_type) => match session_type.as_str() {
	// 		"x11" => match get_text_on_x11() {
	// 			Ok(text) => return text,
	// 			Err(err) => return String::new(),
	// 		},
	// 		"wayland" => match get_text_on_wayland() {
	// 			Ok(text) => return text,
	// 			Err(err) => return String::new(),
	// 		},
	// 		_ => {
	// 			eprintln!("Unknown Session Type: {session_type}");
	// 			return String::new();
	// 		}
	// 	},
	// 	Err(_err) => {
	// 		return String::new();
	// 	}
	// }
	String::new()
}

fn get_text_on_x11() -> Result<String, String> {
	if let Ok(clipboard) = Clipboard::new() {
		if let Ok(primary) = clipboard.load(
			clipboard.getter.atoms.primary,
			clipboard.getter.atoms.utf8_string,
			clipboard.getter.atoms.property,
			Duration::from_millis(100),
		) {
			let result = String::from_utf8_lossy(&primary)
				.trim_matches('\u{0}')
				.trim()
				.to_string();
			Ok(result)
		} else {
			Err("Clipboard Read Failed".to_string())
		}
	} else {
		Err("Clipboard Create Failed".to_string())
	}
}

fn get_text_on_wayland() -> Result<String, String> {
	if let Ok(support) = is_primary_selection_supported() {
		if !support {
			std::env::set_var("XDG_SESSION_TYPE", "x11");
			std::env::set_var("GDK_BACKEND", "x11");
			return get_text_on_x11();
		}
	} else {
		std::env::set_var("XDG_SESSION_TYPE", "x11");
		std::env::set_var("GDK_BACKEND", "x11");
		return get_text_on_x11();
	}

	let result = get_contents(ClipboardType::Primary, Seat::Unspecified, MimeType::Text);

	match result {
		Ok((mut pipe, _)) => {
			let mut contents = vec![];
			pipe.read_to_end(&mut contents).unwrap();
			let contents = String::from_utf8_lossy(&contents)
				.trim_matches('\u{0}')
				.trim()
				.to_string();
			return Ok(contents);
		}

		Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {
			return Ok("".to_string());
		}

		Err(err) => return Err(err.to_string()),
	}
}
