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
    

