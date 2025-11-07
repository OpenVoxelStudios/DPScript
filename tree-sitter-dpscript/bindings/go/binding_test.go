package tree_sitter_dpscript_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_dpscript "github.com/openvoxelstudios/dpscript/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_dpscript.Language())
	if language == nil {
		t.Errorf("Error loading DPScript grammar")
	}
}
