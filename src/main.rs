use std::io;

use niri_ipc::{OutputAction, OutputConfigChanged, Reply, Request, Response, socket::Socket};

fn main() -> Result<(), ToggleWindowError> {
    let mut socket = Socket::connect()?;
    let response = socket
        .send(Request::Outputs)?
        .map_err(ToggleWindowError::Niri)?;
    let Response::Outputs(outputs) = response else {
        return Err(ToggleWindowError::InvalidResponse(
            "Response was not an Response::Outputs",
        ));
    };
    if !outputs.len() == 2 {
        return Err(ToggleWindowError::InvalidResponse(
            "Wrong number of outputs",
        ));
    }
    let Some(disabled_output) = outputs.iter().find_map(|(key, value)| {
        if value.current_mode.is_none() {
            Some(key)
        } else {
            None
        }
    }) else {
        return Err(ToggleWindowError::InvalidResponse(
            "Missing disabled output",
        ));
    };
    let Some(enabled_output) = outputs.iter().find_map(|(key, value)| {
        if value.current_mode.is_some() {
            Some(key)
        } else {
            None
        }
    }) else {
        return Err(ToggleWindowError::InvalidResponse("Missing enabled output"));
    };

    let response = socket.send(Request::Output {
        output: disabled_output.clone(),
        action: OutputAction::On,
    })?;
    validate_output_change_response(response)?;

    let response = socket.send(Request::Output {
        output: enabled_output.clone(),
        action: OutputAction::Off,
    })?;
    println!("Disable {enabled_output}:{response:?}");
    validate_output_change_response(response)?;
    Ok(())
}

fn validate_output_change_response(reply: Reply) -> Result<(), ToggleWindowError> {
    match reply.map_err(ToggleWindowError::Niri)? {
        Response::OutputConfigChanged(OutputConfigChanged::Applied) => {}
        Response::OutputConfigChanged(OutputConfigChanged::OutputWasMissing) => {
            return Err(ToggleWindowError::FailedToApplyChange("Output was missing"));
        }
        _ => {
            return Err(ToggleWindowError::InvalidResponse(
                "Invalid response for enabling output",
            ));
        }
    };
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum ToggleWindowError {
    #[error("Io error:{0}")]
    Io(#[from] io::Error),
    #[error("Niri Error:{0}")]
    Niri(String),
    #[error("Niri response invalid:{0}")]
    InvalidResponse(&'static str),
    #[error("Failed to apply change:{0}")]
    FailedToApplyChange(&'static str),
}
