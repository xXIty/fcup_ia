from enum import Enum, auto
from queue import Queue
from config import Config, TreeNode

# Algorithm enumeration
class QueueingFunction(Enum):
    DFS     =  auto()
    BFS     =  auto()
    IDFS    =  auto()
    GULOSA  =  auto()
    A_STAR  =  auto()



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

def insert(descendantList, queue, queueingFunction):
    queueingFunction(descendantList, queue)

def GeneralSearchAlgorithm(queueingFunction, configInicial, configFinal):
    #if thereIsNoSolution(configInicial, configFinal):
    #    return "It is impossible to reach a solution"
    queue = Queue()
    queue.put(TreeNode(configInicial))

    while not queue.empty():
        node = queue.get()
        if node.data == configFinal:
            # return Path to solution
            return node
        descendantList = makeDescendants(node)
        insert(descendantList, queue, queueingFunction)
    return "solution not found"

def iterativaProfundidad(descendantList, queue):
    for node in descendantList:
        queue.put(node) 

def printPath(node):
    print(node.data)
    if node.parent != None:
        printPath(node.parent)
        
if __name__ == "__main__":
    N = 4
    #queueingFunction = QueueingFunction.DFS
    queueingFunction = iterativaProfundidad

    FSC        =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0])
    confA      =  Config(N,  [1,2,3,4,5,6,8,12,13,9,0,7,14,11,10,15])
    confB      =  Config(N,  [1,2,3,4,13,6,8,12,5,9,0,7,14,11,10,15])
    confLink1  =  Config(N,  [12,1,10,2,7,11,4,14,5,0,9,15,8,13,6,3])
    confini    =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,0,15])


    #print(thereIsNoSolution(confA,FSC))
    #print(thereIsNoSolution(confB,FSC))
    #print(thereIsNoSolution(confLink1,FSC))

    #print(confA)
    #for c in confA.getSuccessors():
      #print(c.getInv())
      #print(c)

    #confLink1.getInv()
    configInicial = confini
    configFinal   = FSC
    searchResult = GeneralSearchAlgorithm( queueingFunction,
                                           configInicial,
                                           configFinal)

    if isinstance(searchResult, str):
        print(searchResult)
    else:
        printPath(searchResult)


