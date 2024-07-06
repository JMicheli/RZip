import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { App, Button, Checkbox, Flex, Input } from "antd";
import FileTree from "./FileTree";

function UiRoot() {
  const [sourcePath, setSourcePath] = useState("");
  const [deleteAfterExtract, setDeleteAfterExtract] = useState(false);
  const [enableOutputPath, setEnableOutputPath] = useState(false);
  const [outputPath, setOutputPath] = useState("");

  async function start_clicked() {
    console.log("Start clicked");
    invoke("start_extraction");
  }

  async function refresh_clicked() {
    console.log("Refresh clicked");
    invoke("do_refresh");
  }

  return (
    <App style={{ padding: "5px", height: "100vh" }}>
      <Flex vertical gap={10} style={{ height: "100%" }}>
        <Flex gap={10} justify="space-between" style={{ flexGrow: 1, minHeight: 0 }}>
          {/* Target column */}
          <Flex vertical gap={10} style={{ width: "50%", overflow: "auto" }}>
            <Input
              placeholder="Source Path"
              value={sourcePath}
              onChange={(e) => setSourcePath(e.target.value)}
            />
            <FileTree />
          </Flex>
          {/* Output column */}
          <Flex vertical gap={10} style={{ width: "50%", overflow: "auto" }}>
            <Input
              placeholder="Output Path"
              value={outputPath}
              onChange={(e) => setOutputPath(e.target.value)}
            />
            <FileTree />
          </Flex>
        </Flex>

        {/* Bottom controls */}
        <Flex gap="10px" justify="flex-end">
          <Checkbox
            value={deleteAfterExtract}
            onChange={() => setDeleteAfterExtract(!deleteAfterExtract)}
          >
            Delete archives after extraction
          </Checkbox>
          <Button type="primary" onClick={start_clicked}>
            Start
          </Button>
          <Button onClick={refresh_clicked}>Refresh</Button>
        </Flex>
      </Flex>
    </App>
  );
}

export default UiRoot;
