import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Tree, TreeDataNode } from "antd";
import { CarryOutOutlined, CheckOutlined, FormOutlined } from "@ant-design/icons";

const treeData: TreeDataNode[] = [
  {
    title: "parent 1",
    key: "0-0",
    icon: <CarryOutOutlined />,
    children: [
      {
        title: "parent 1-0",
        key: "0-0-0",
        icon: <CarryOutOutlined />,
        children: [
          { title: "leaf", key: "0-0-0-0", icon: <CarryOutOutlined /> },
          {
            title: (
              <>
                <div>multiple line title</div>
                <div>multiple line title</div>
              </>
            ),
            key: "0-0-0-1",
            icon: <CarryOutOutlined />
          },
          { title: "leaf", key: "0-0-0-2", icon: <CarryOutOutlined /> }
        ]
      },
      {
        title: "parent 1-1",
        key: "0-0-1",
        icon: <CarryOutOutlined />,
        children: [{ title: "leaf", key: "0-0-1-0", icon: <CarryOutOutlined /> }]
      },
      {
        title: "parent 1-2",
        key: "0-0-2",
        icon: <CarryOutOutlined />,
        children: [
          { title: "leaf", key: "0-0-2-0", icon: <CarryOutOutlined /> },
          {
            title: "leaf",
            key: "0-0-2-1",
            icon: <CarryOutOutlined />,
            switcherIcon: <FormOutlined />
          }
        ]
      }
    ]
  },
  {
    title: "parent 2",
    key: "0-1",
    icon: <CarryOutOutlined />,
    children: [
      {
        title: "parent 2-0",
        key: "0-1-0",
        icon: <CarryOutOutlined />,
        children: [
          { title: "leaf", key: "0-1-0-0", icon: <CarryOutOutlined /> },
          { title: "leaf", key: "0-1-0-1", icon: <CarryOutOutlined /> }
        ]
      }
    ]
  }
];

function FileTree() {
  const [showLine, setShowLine] = useState<boolean>(true);
  const [showIcon, setShowIcon] = useState<boolean>(false);
  const [showLeafIcon, setShowLeafIcon] = useState<React.ReactNode>(true);

  const onSelect = (selectedKeys: React.Key[], info: any) => {
    console.log("selected", selectedKeys, info);
  };

  const handleLeafIconChange = (value: "true" | "false" | "custom") => {
    if (value === "custom") {
      return setShowLeafIcon(<CheckOutlined />);
    }

    if (value === "true") {
      return setShowLeafIcon(true);
    }

    return setShowLeafIcon(false);
  };

  return (
    <div>
      <Tree
        showLine={showLine ? { showLeafIcon } : false}
        showIcon={showIcon}
        defaultExpandedKeys={["0-0-0"]}
        onSelect={onSelect}
        treeData={treeData}
      />
    </div>
  );
}

export default FileTree;
