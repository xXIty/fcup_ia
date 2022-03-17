from enum import Enum, auto
from queue import Queue

# Algorithm enumeration
class QueueingFunction(Enum):
    DFS     =  auto()
    BFS     =  auto()
    IDFS    =  auto()
    GULOSA  =  auto()
    A_STAR  =  auto()

class TreeNode:
    def __init__(self,data):
        self.data      =  data
        self.children  =  []
        slef.parent    =  None

class Config:
    
    def __init__(self, N, board):
        self.N          =  N                  
        self.board      =  board              
        self.blank      =  board.index(0)     
        self.blankModN  =  self.blank % self.N

    def __str__(self):
        output = ''
        for i in range(self.N):
            for j in range(self.N):
                output += "\t{}".format(self.board[i*self.N + j])
            output += '\n'
        output += '\n'
        return output

    # Get the number of inversions of a the board configuration.
    def getInv(self):
        inv = 0
        boardt = self.board.copy()
        boardt.remove(0)

        for i in range(N*N-1):
            invi = 0
            val = boardt[i]
            for j in range(i+1,N*N-1):
                if boardt[i] > boardt[j]:
                    invi = invi + 1
            inv = inv + invi

        return inv

    # Swap the blank space blankDiff positions in the board array.
    def move(self, blankDiff):
        blankNew = self.blank + blankDiff

        # Swap values
        val = self.board[blankNew]
        self.board[blankNew] = 0
        self.board[self.blank] = val
        self.blank = blankNew

    def moveUp(self):
        # The upper position has an offset of -N from the blank space in the board array.
        self.move(-self.N)

    def moveDown(self):
        # The lower position has an offset of +N from the blank space in the board array.
        self.move(self.N)

    def moveRight(self):
        # The right position has an offset of +1 from the blank space in the board array.
        self.move(1)

    def moveLeft(self):
        # The left position has an offset of -1 from the blank space in the board array.
        self.move(-1)

    def canMoveUp(self):
        return self.blank > (self.N - 1)

    def canMoveDown(self):
        return self.blank < (self.N * self.N - N)

    def canMoveLeft(self):
        return self.blankModN > 0

    def canMoveRight(self):
        return self.blankModN < self.N-1

    def getSuccessors(self):
        childs = []
        if self.canMoveLeft():
            configNew = Config(self.N, self.board.copy())
            configNew.moveLeft()
            childs.append(configNew)
        if self.canMoveRight():
            configNew = Config(self.N, self.board.copy())
            configNew.moveRight()
            childs.append(configNew)
        if self.canMoveUp():
            configNew = Config(self.N, self.board.copy())
            configNew.moveUp()
            childs.append(configNew)
        if self.canMoveDown():
            configNew = Config(self.N, self.board.copy())
            configNew.moveDown()
            childs.append(configNew)
        return childs


def thereIsNoSolution(configIni, configFin):
      invi = configIni.getInv()  
      invf = configFin.getInv()  

      blankRowi = configIni.N - int(configIni.blank / configIni.N)
      blankRowf = configFin.N - int(configFin.blank / configFin.N)

      condi = (invi%2 == 0) == ( blankRowi%2 == 1)
      condf = (invf%2 == 0) == ( blankRowf%2 == 1)

      return condi != condf

def GeneralSearchAlgorithm(queueingFunction, configInicial, configFinal):
    if thereIsNoSolution(configInicial, configFinal):
        return "It is impossible to reach a solution"
    queue = Queue()

    while not queue.empty():
        node = queue.pop()
        if node.config.is_solved

if __name__ == "__main__":
    N = 4
    queueingFunction = QueueingFunction.DFS

    FSC        =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0])
    confA      =  Config(N,  [1,2,3,4,5,6,8,12,13,9,0,7,14,11,10,15])
    confB      =  Config(N,  [1,2,3,4,13,6,8,12,5,9,0,7,14,11,10,15])
    confLink1  =  Config(N,  [12,1,10,2,7,11,4,14,5,0,9,15,8,13,6,3])


    print(thereIsNoSolution(confA,FSC))
    print(thereIsNoSolution(confB,FSC))
    print(thereIsNoSolution(confLink1,FSC))

    print(confA)
    for c in confA.getSuccessors():
      print(c.getInv())
      print(c)

    #confLink1.getInv()
    configInicial = confB
    configFinal   = FSC
    searchResult = GeneralSearchAlgorithm( queueingFunction,
                                           configInicial,
                                           configFinal)

    if isinstance(searchResult, str):
        print(searchResult)


