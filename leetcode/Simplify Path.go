import p "path"

func simplifyPath(path string) string {
	return p.Clean(path)
}
