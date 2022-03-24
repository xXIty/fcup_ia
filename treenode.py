
treeInsertCounter = 0
tree = None
videoWriter = None

DEBUG = False
#DEBUG = True

class TreeNode:
    def __init__(self,data, parent=None):
        self.data      =  data
        self.children  =  []
        self.parent    =  parent
        self.depth     =  None
        if parent != None:
            self.depth = parent.depth + 1
        else:
            self.depth = 0

    def __eq__(self, other):
        if other != None:
            return self.data == other.data
        return False
    

def initTree(root):
    if not DEBUG:
        return
    from ete3 import Tree, TextFace, NodeStyle, TreeStyle
    from cv2 import VideoWriter, VideoWriter_fourcc, imread, imshow
    global tree
    global treeInsertCounter
    global videoWriter
    tree = Tree()
    videoWriter = VideoWriter('outputs/output.avi', VideoWriter_fourcc('M','J','P','G'), 10, (600, 600))
    treeInsertCounter = 0
    tree = Tree()
    tree.add_child(name=str(id(root)))
    face = TextFace(str(root.data), ftype="Courier")
    tree.add_face(face, column=0)


def printTree():
    if not DEBUG:
        return
    ts = TreeStyle()
    ts.show_leaf_name = False
    tree.show(tree_style=ts)
    #tree.render('outputs/frame.png', w=600, h=600, tree_style=ts)
    #frame = imread('outputs/frame.png')
    #global videoWriter
    #videoWriter.write(frame)


def updateTreePaint(node):
    if not DEBUG:
        return
    if node.parent is not None:
        global treeInsertCounter
        treeInsertCounter += 1
        parentNode = tree & str(id(node.parent))
        parentNode.add_child(name=str(id(node)))
        face = TextFace(str(node.data)+'\n'+str(node.data.heuristic + node.depth), ftype="Courier")
        nodePainted = tree & str(id(node)) 
        nodePainted.add_face(face, column=0)
        printTree()
