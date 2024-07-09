import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { App, Button, Checkbox, Flex, Input } from "antd";
import { FolderOutlined, RedoOutlined, DoubleRightOutlined } from "@ant-design/icons";
import FileTree from "../components/FileTree";
import { open } from "@tauri-apps/api/dialog";
import PathInput from "../components/PathInput";

function RzipApp() {
  const [sourcePath, setSourcePath] = useState(".");
  const [deleteAfterExtract, setDeleteAfterExtract] = useState(false);
  const [enableOutputPath, setEnableOutputPath] = useState(false);
  const [outputPath, setOutputPath] = useState(".");

  async function start_clicked() {
    console.log("Start clicked");
    invoke("start_extraction");
  }

  async function refresh_clicked() {
    console.log("Refresh clicked");
    invoke("do_refresh");
  }

  async function source_file_dialog_clicked() {
    console.log("Source dialog callback");

    open({
      directory: true,
      multiple: false
    });
  }

  return (
    <App style={{ padding: "5px", height: "100vh" }}>
      <Flex vertical gap={10} style={{ height: "100%" }}>
        <Flex gap={10} justify="space-between" style={{ flexGrow: 1, minHeight: 0 }}>
          {/* Target column */}
          <Flex vertical gap={10} style={{ width: "50%", overflow: "auto" }}>
            <PathInput
              placeholder="Source Path"
              onPathSelect={setSourcePath}
              settings={{
                allow_nonexistent: true,
                allow_directories: true,
                allow_files: false
              }}
            />
            <FileTree path={sourcePath} />
          </Flex>
          {/* Output column */}
          <Flex vertical gap={10} style={{ width: "50%", overflow: "auto" }}>
            <PathInput
              placeholder="Output Path"
              onPathSelect={setOutputPath}
              settings={{
                allow_nonexistent: true,
                allow_directories: true,
                allow_files: false
              }}
            />
            <FileTree path={outputPath} />
          </Flex>
        </Flex>

        {/* Bottom controls */}
        <Flex gap="10px" justify="flex-end">
          <Checkbox
            checked={deleteAfterExtract}
            onChange={() => setDeleteAfterExtract(!deleteAfterExtract)}
          >
            Delete archives after extraction
          </Checkbox>
          <Button type="primary" icon={<DoubleRightOutlined />} onClick={start_clicked}>
            Start
          </Button>
          <Button icon={<RedoOutlined />} onClick={refresh_clicked}>
            Refresh
          </Button>
        </Flex>
      </Flex>
    </App>
  );
}

export default RzipApp;
