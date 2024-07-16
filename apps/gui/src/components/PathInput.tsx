import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Input, message } from "antd";
import { FolderOutlined } from "@ant-design/icons";

interface PathInputProps {
  onPathSelect: (path: string) => void;
  placeholder?: string;
  settings: ValidationSettings;
}

interface ValidationSettings {
  allow_files: boolean;
  allow_directories: boolean;
  allow_nonexistent: boolean;
}

function PathInput({ onPathSelect, placeholder, settings }: PathInputProps) {
  const [currentPath, setCurrentPath] = useState<string>(".");

  async function open_dialog_clicked() {
    // Send open dialog via Tauri
    const selected_paths = await open({
      directory: settings.allow_directories,
      multiple: false
    });

    // Handle response from open dialog
    if (selected_paths) {
      // Deal with array types
      const selected_path = Array.isArray(selected_paths) ? selected_paths[0] : selected_paths;
      // Validate and handle
      const is_valid = await validate_path(selected_path);
      if (is_valid) {
        // Set stored path and invoke external handler
        setCurrentPath(selected_path);
        onPathSelect(selected_path);
      } else {
        message.error("Path is not valid.");
      }
    }
  }

  /**
   * Validates a path by sending it to the backend, checks the path against
   * the settings supplied.
   * @param path The path sent to the backend for validation.
   */
  async function validate_path(path: string): Promise<boolean> {
    return invoke("validate_path", { path: path, settings: settings });
  }

  async function on_path_changed(e: React.ChangeEvent<HTMLInputElement>) {
    const new_path = e.target.value;
    setCurrentPath(new_path);

    // Check path validity with backend
    const is_valid = await validate_path(new_path);
    if (is_valid) {
      // Invoke external handler
      onPathSelect(new_path);
    } else {
      message.error("Path is not valid.");
    }
  }

  return (
    <Input
      addonAfter={<FolderOutlined onClick={open_dialog_clicked} />}
      placeholder={placeholder}
      value={currentPath}
      onChange={on_path_changed}
    />
  );
}

export default PathInput;
