from ete3 import Tree

tree = Tree()
class TreeNode:
    def __init__(self,data, parent=None):
        self.data      =  data
        self.children  =  []
        self.parent    =  parent

def initTree(root):
    tree.add_child(name=str(root.data))


def printTree():
    tree.show()

