import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button, Checkbox, Col, Flex, Input, Row, Tree } from "antd";
import FileTree from "./FileTree";

function App() {
  const [sourcePath, setSourcePath] = useState("");
  const [deleteAfterExtract, setDeleteAfterExtract] = useState(false);
  const [enableOutputPath, setEnableOutputPath] = useState(false);
  const [outputPath, setOutputPath] = useState("");

  async function start_extraction() {
    console.log("Start clicked");
  }

  async function refresh() {
    console.log("Refresh clicked");
  }

  function target_column() {
    return (
      <Flex vertical gap={12}>
        <Input placeholder="Outlined" />
        <FileTree />
      </Flex>
    );
  }

  function output_column() {
    return (
      <Flex vertical gap={12}>
        <Input placeholder="Outlined" />
        <FileTree />
      </Flex>
    );
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
        <Button>Start</Button>
        <Button>Refresh</Button>
      </Flex>
    );
  }

  return (
    <div className="app-container">
      <Row gutter={10}>
        <Col span={12}>{target_column()}</Col>
        <Col span={12}>{output_column()}</Col>
      </Row>
      <Row>
        <Col span={24}>{bottom_bar()}</Col>
      </Row>
    </div>
  );
}

export default App;
