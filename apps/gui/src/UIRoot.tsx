import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { App, Button, Checkbox, Col, Flex, Input, Row, Tree } from "antd";
import FileTree from "./FileTree";

function UIRoot() {
  const [sourcePath, setSourcePath] = useState("");
  const [deleteAfterExtract, setDeleteAfterExtract] = useState(false);
  const [enableOutputPath, setEnableOutputPath] = useState(false);
  const [outputPath, setOutputPath] = useState("");

  async function start_clicked() {
    console.log("Start clicked");
  }

  async function refresh_clicked() {
    console.log("Refresh clicked");
  }

  function bottom_bar() {
    return (
      <Flex gap="10px" justify="flex-end">
        <Checkbox
          value={deleteAfterExtract}
          onChange={() => setDeleteAfterExtract(!deleteAfterExtract)}
        >
          Delete archives after extraction
        </Checkbox>
        <Button onClick={start_clicked}>Start</Button>
        <Button onClick={refresh_clicked}>Refresh</Button>
      </Flex>
    );
  }

  return (
    <App>
      <Flex vertical gap={10}>
        <Flex gap={10}>
          {/* Target column */}
          <Col span={12}>
            <Flex vertical gap={12}>
              <Input placeholder="Outlined" />
              <FileTree />
            </Flex>
          </Col>
          {/* Output column */}
          <Col span={12}>
            <Flex vertical gap={12}>
              <Input placeholder="Outlined" />
              <FileTree />
            </Flex>
          </Col>
        </Flex>

        {/* Bottom controls */}
        {bottom_bar()}
      </Flex>
    </App>
  );
}

export default UIRoot;
