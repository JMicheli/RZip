import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Tree, TreeDataNode } from "antd";
import { CarryOutOutlined, CheckOutlined, FormOutlined, FileOutlined } from "@ant-design/icons";

const iconMapper: Record<string, JSX.Element> = {
  CarryOutOutlined: <CarryOutOutlined />,
  CheckOutlined: <CheckOutlined />,
  FormOutlined: <FormOutlined />,
  FileOutlined: <FileOutlined />
};

const getIcon = (iconName?: string) => {
  return iconName ? iconMapper[iconName] : undefined;
};

const mapIconsRecursively = (node: TreeDataNode): TreeDataNode => {
  return {
    ...node,
    icon: getIcon(node.icon as string),
    children: node.children ? node.children.map(mapIconsRecursively) : undefined
  };
};

interface FileTreeProps {
  path: string;
}

function FileTree({ path }: FileTreeProps) {
  const [treeData, setTreeData] = useState([] as TreeDataNode[]);

  useEffect(() => {
    invoke("get_file_tree", { path: path })
      .then((data) => {
        const typed_data = data as TreeDataNode[];
        const mappedData = typed_data.map(mapIconsRecursively);

        setTreeData(mappedData);
      })
      .catch((error) => console.error("Error fetching file tree:", error));
  }, [path]);

  return (
    <Tree
      showLine={{ showLeafIcon: false }}
      showIcon
      defaultExpandedKeys={["0-0-0"]}
      onSelect={(selectedKeys, info) => console.log("selected", selectedKeys, info)}
      treeData={treeData}
    />
  );
}

export default FileTree;
