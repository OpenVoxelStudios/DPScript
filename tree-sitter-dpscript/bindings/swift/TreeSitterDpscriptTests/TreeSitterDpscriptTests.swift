import XCTest
import SwiftTreeSitter
import TreeSitterDpscript

final class TreeSitterDpscriptTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_dpscript())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading DPScript grammar")
    }
}
