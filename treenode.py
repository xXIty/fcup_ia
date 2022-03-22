from ete3 import Tree, TextFace, NodeStyle, TreeStyle
from cv2 import VideoWriter, VideoWriter_fourcc, imread, imshow

tree = Tree()
videoWriter = VideoWriter('outputs/output.avi', VideoWriter_fourcc('M','J','P','G'), 10, (600, 600))
treeInsertCounter = 0
class TreeNode:
    def __init__(self,data, parent=None):
        self.data      =  data
        self.children  =  []
        self.parent    =  parent

def initTree(root):
    global tree
    global treeInsertCounter
    global videoWriter
    treeInsertCounter = 0
    tree = Tree()
    tree.add_child(name=str(id(root)))
    face = TextFace(str(root.data), ftype="Courier")
    tree.add_face(face, column=0)


def printTree():
    ts = TreeStyle()
    ts.show_leaf_name = False
    #tree.show(tree_style=ts)
    tree.render('outputs/frame.png', w=600, h=600, tree_style=ts)
    frame = imread('outputs/frame.png')
    global videoWriter
    videoWriter.write(frame)


def updateTreePaint(node):
    if node.parent is not None:
        global treeInsertCounter
        treeInsertCounter += 1
        parentNode = tree & str(id(node.parent))
        parentNode.add_child(name=str(id(node)))
        face = TextFace(str(node.data)+'\n'+str(treeInsertCounter), ftype="Courier")
        nodePainted = tree & str(id(node)) 
        nodePainted.add_face(face, column=0)
        printTree()
