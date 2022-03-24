from  enum      import  Enum, auto
from  queue     import  Queue     

from  config    import  Config    
from  treenode  import  *



nodeCount = None
heuristic = 0

def getNodeCount():
    global nodeCount
    return nodeCount

def setHeuristic(val):
    global heuristic
    heuristic = val

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
            else:
                del successor
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

def depthFirstSearch(descendantList, queue, configFinal):
    global nodeCount
    for descendant in descendantList:
        if nodeCreatesCycle(descendant):
            del descendant
            continue
        else:
            queue.insert(0,descendant)
            updateTreePaint(descendant)
            nodeCount += 1

def iterativeDepthFirstSearchRecursive(maxDepth, currentDepth, node, configFinal):
    updateTreePaint(node)

    if node.data == configFinal:
        return node
    if currentDepth == maxDepth:
        return None

    descendantList = makeDescendants(node)
    for descendant in descendantList:

        if nodeCreatesCycle(descendant):
            del descendant
            continue

        # Continue DFS through this descendant
        else: 
            global nodeCount
            nodeCount += 1
            result  =  iterativeDepthFirstSearchRecursive(  maxDepth,
                                                            currentDepth+1,
                                                            descendant,
                                                            configFinal)
            if result is  None:
                continue
            else:
                return result


def iterativeDepthFirstSearch(configInicial, configFinal):
    if thereIsNoSolution(configInicial, configFinal):
        return "It is impossible to reach a solution"

    result = None
    depth = 1
    root = TreeNode(configInicial)
    global nodeCount
    nodeCount = 1

    while result is None:
        initTree(root)
        depth += 1
        result = iterativeDepthFirstSearchRecursive(depth, 0, root, configFinal)
    return result

def breadthFirstSearch(descendantList, queue, configFinal):
    global nodeCount
    for descendant in descendantList:
        if nodeCreatesCycle(descendant):
            del descendant
            continue
        else:
            queue.append(descendant)
            updateTreePaint(descendant)
            nodeCount += 1

def greedySearch(descendantList, queue, configFinal):
    global nodeCount
    for descendant in descendantList:
        if nodeCreatesCycle(descendant):
            del descendant
            continue
        else:
            global heuristic
            if heuristic == 0:
                descendant.data.setHeuristicManhattan(configFinal)
            else:
                descendant.data.setHeuristicMisplaced(configFinal)
            queue.append(descendant)
            updateTreePaint(descendant)
            nodeCount += 1
    minNode = min(queue, key=lambda node: node.data.heuristic)
    indexMin = queue.index(minNode)
    queue.pop(indexMin)
    queue.insert(0, minNode)
        

def aStarSearch(descendantList, queue, configFinal):
    global nodeCount
    for descendant in descendantList:
        if nodeCreatesCycle(descendant):
            del descendant
            continue
        else:
            global heuristic
            if heuristic == 0:
                descendant.data.setHeuristicManhattan(configFinal)
            else:
                descendant.data.setHeuristicMisplaced(configFinal)
            queue.append(descendant)
            updateTreePaint(descendant)
            nodeCount += 1
    minNode = min(queue, key=lambda node: (node.data.heuristic + node.depth))
    indexMin = queue.index(minNode)
    queue.pop(indexMin)
    queue.insert(0, minNode)

def insert(descendantList, queue, queueingFunction, configFinal):
    queueingFunction(descendantList, queue, configFinal)

def GeneralSearchAlgorithm(queueingFunction, configInicial, configFinal):
    if thereIsNoSolution(configInicial, configFinal):
        return "It is impossible to reach a solution"

    # Init Tree
    # Using a list for the queue allows to use it as queue or stack.
    queue = [TreeNode(configInicial)]
    initTree(queue[0])
    global nodeCount
    nodeCount = 1

    while queue:
        node = queue.pop(0)
        if node.data == configFinal:
            # return Path to solution
            #printTree()
            return node
        descendantList = makeDescendants(node)
        insert(descendantList, queue, queueingFunction, configFinal)
    return "solution not found"

# Return depth of node and str representation of each step from the root.
def getPathString(node):
    path = []
    path.insert(0,node)
    pathString = ''

    while node.parent != None:
        node = node.parent
        path.insert(0,node)
    
    for node in path:
        pathString += str(node.data)

    return pathString

# Algorithm enumeration
algoritmsDict = {    "DFS"     :  depthFirstSearch,
                     "BFS"     :  breadthFirstSearch,
                     "IDFS"    :  iterativeDepthFirstSearch,
                     "GREEDY_MIS"  :  greedySearch,
                     "GREEDY_MAN"  :  greedySearch,
                     "ASTAR_MIS"   :  aStarSearch,
                     "ASTAR_MAN"   :  aStarSearch }

