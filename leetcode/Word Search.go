package leetcode

type searchTree struct {
	board     [][]byte
	word      string
	wordIndex int
	visited   [][]bool
}

func (s *searchTree) reset() *searchTree {
	s.wordIndex = 0

	s.visited = make([][]bool, len(s.board))
	for i := range s.visited {
		s.visited[i] = make([]bool, len(s.board[0]))
	}

	return s
}

func (s *searchTree) find(h, v int) bool {
	if h < 0 || h >= len(s.board) || v < 0 || v >= len(s.board[0]) {
		return false
	}

	if s.board[h][v] != s.word[s.wordIndex] {
		return false
	}

	if s.visited[h][v] {
		return false
	}

	if s.wordIndex == len(s.word)-1 {
		return true
	}

	s.visited[h][v] = true
	s.wordIndex++

	found := s.find(h+1, v) ||
		s.find(h-1, v) ||
		s.find(h, v+1) ||
		s.find(h, v-1)

	s.visited[h][v] = false
	s.wordIndex--

	return found
}

func exist(board [][]byte, word string) bool {
	if len(board) == 0 || len(board[0]) == 0 {
		return false
	}

	s := &searchTree{
		board: board,
		word:  word,
	}

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if board[i][j] == word[0] && s.reset().find(i, j) {
				return true
			}
		}
	}

	return false
}
