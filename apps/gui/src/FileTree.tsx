import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Tree, TreeDataNode } from "antd";
import { CarryOutOutlined, CheckOutlined, FormOutlined } from "@ant-design/icons";

const iconMapper: Record<string, JSX.Element> = {
  CarryOutOutlined: <CarryOutOutlined />,
  CheckOutlined: <CheckOutlined />,
  FormOutlined: <FormOutlined />
};

const getIcon = (iconName?: string) => {
  return iconName ? iconMapper[iconName] : undefined;
};
function FileTree() {
  const [treeData, setTreeData] = useState([] as TreeDataNode[]);

  useEffect(() => {
    invoke("get_file_tree")
      .then((data) => {
        let typed_data = data as any[];
        const mappedData = typed_data.map((node) => ({
          ...node,
          icon: getIcon(node.icon),
          children: node.children
            ? node.children.map((child: { icon: string | undefined }) => ({
                ...child,
                icon: getIcon(child.icon)
              }))
            : undefined
        }));
        setTreeData(mappedData);
      })
      .catch((error) => console.error("Error fetching file tree:", error));
  }, []);

  return (
    <div>
      <Tree
        showLine={{ showLeafIcon: true }}
        showIcon
        defaultExpandedKeys={["0-0-0"]}
        onSelect={(selectedKeys, info) => console.log("selected", selectedKeys, info)}
        treeData={treeData}
      />
    </div>
  );
}

export default FileTree;
