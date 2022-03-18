from  enum      import  Enum, auto
from  queue     import  Queue     

from  config    import  Config    
from  treenode  import  TreeNode  

# Algorithm enumeration
class QueueingFunction(Enum):
    DFS     =  auto()
    # BFS     =  auto()
    # IDFS    =  auto()
    # GULOSA  =  auto()
    # A_STAR  =  auto()

def thereIsNoSolution(configIni, configFin):
      invi = configIni.getInv()  
      invf = configFin.getInv()  

      blankRowi = configIni.N - int(configIni.blank / configIni.N)
      blankRowf = configFin.N - int(configFin.blank / configFin.N)

      condi = (invi%2 == 0) == ( blankRowi%2 == 1)
      condf = (invf%2 == 0) == ( blankRowf%2 == 1)

      return condi != condf

def makeDescendants(node):
        descendantList = []
        for successor in node.data.getSuccessors():
            if node.parent == None or successor != node.parent.data:
                descendantList.append(TreeNode(successor,node))
        node.children = descendantList
        return descendantList

def nodeCreatesCycle(node):
    cycleExists  =  False
    nodeInPath   =  node
    while nodeInPath.parent != None:
        nodeInPath = nodeInPath.parent
        if nodeInPath.data == node.data:
            cycleExists = True
            break
    return cycleExists

def depthFirstSearch(descendantList, queue):
    for descendant in descendantList:
        if nodeCreatesCycle(descendant):
            continue
        else:
            queue.insert(0,descendant)

def insert(descendantList, queue, queueingFunction):
    match queueingFunction:
        case DFS:
            depthFirstSearch(descendantList, queue)

def GeneralSearchAlgorithm(queueingFunction, configInicial, configFinal):
    if thereIsNoSolution(configInicial, configFinal):
        return "It is impossible to reach a solution"

    # Using a list for the queue allows to use it as queue or stack.
    queue = [TreeNode(configInicial)]

    while queue:
        node = queue.pop()
        if node.data == configFinal:
            # return Path to solution
            return node
        descendantList = makeDescendants(node)
        insert(descendantList, queue, queueingFunction)
    return "solution not found"

def printPath(node):
    path = []
    path.insert(0,node)

    while node.parent != None:
        node = node.parent
        path.insert(0,node)
    
    for node in path:
        print(node.data)
