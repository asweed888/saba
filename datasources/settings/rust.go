package settings

import (
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"strings"

	"github.com/asweed888/saba/domain/model"
	"golang.org/x/xerrors"
)

type rustSubAction struct{}

var Rust = &model.TacitSetting{
	Ext:         "rs",
	FileModeStr: "0644",
	TacitSubAction: func(decl *model.Declare) error {

		subaction := &rustSubAction{}

		filePath := subaction.targetFile()
		searchString := subaction.modBlockRegExp()
		insertString := subaction.modBlock(decl)

		// ファイル読み込み
		fileData, err := ioutil.ReadFile(filePath)
		if err != nil {
			return xerrors.Errorf("%+v", err)
		}

		// 正規表現で文字列を検索
		re := regexp.MustCompile(searchString)
		if re.Match(fileData) {
			// 指定した文字列が存在する場合、差し替えを行う
			newData := re.ReplaceAll(fileData, []byte(insertString))

			// ファイルに書き込み
			err = ioutil.WriteFile(filePath, newData, os.ModePerm)
			if err != nil {
				return xerrors.Errorf("%+v", err)
			}

		} else {
			// 指定した文字列が存在しない場合、先頭に文字列を追記
			newData := append([]byte(insertString), fileData...)
			// ファイルに書き込み
			err = ioutil.WriteFile(filePath, newData, os.ModePerm)
			if err != nil {
				xerrors.Errorf("%+v", err)
			}
		}

		return nil
	},
}

func (u *rustSubAction) targetFile() string {
	// main.rs ファイルの存在を確認
	if u.checkFileExists("main.rs") {
		return "main.rs"
	}

	// mod.rs ファイルの存在を確認
	if u.checkFileExists("mod.rs") {
		return "mod.rs"
	}

	// main.rs ファイルを作成
	_ = ioutil.WriteFile("main.rs", []byte{}, 0644)

	return "main.rs"
}

func (u *rustSubAction) checkFileExists(filePath string) bool {
	_, err := ioutil.ReadFile(filePath)
	return err == nil
}

func (u *rustSubAction) modBlockRegExp() string {
	return `mod[\s\S]*//.*Automatically exported by saba.`
}

func (u *rustSubAction) modBlock(decl *model.Declare) string {
	var builder strings.Builder

	for idx, spec := range decl.Spec {
		builder.WriteString(fmt.Sprintf("mod %s {\n", spec.Location))

		if len(spec.Upstream) != 0 {
			builder.WriteString(u.upstreamModBlock(spec.Upstream, "    "))
		}

		if len(spec.CodeFile) != 0 {
			builder.WriteString(u.codefileModBlock(spec.CodeFile, "    "))
		}

		if idx == len(decl.Spec)-1 {
			builder.WriteString("} // Automatically exported by saba.")
		} else {
			builder.WriteString("}\n")
		}
	}

	return builder.String()
}

func (u *rustSubAction) upstreamModBlock(upstream []*model.DeclareUpstream, tabs string) string {
	var builder strings.Builder

	for _, ups := range upstream {
		builder.WriteString(fmt.Sprintf("%spub mod %s {\n", tabs, ups.Name))

		if len(ups.Upstream) != 0 {
			builder.WriteString(u.upstreamModBlock(ups.Upstream, fmt.Sprintf("%s    ", tabs)))
		}

		if len(ups.CodeFile) != 0 {
			builder.WriteString(u.codefileModBlock(ups.CodeFile, fmt.Sprintf("%s    ", tabs)))
		}

		builder.WriteString(fmt.Sprintf("%s}\n", tabs))
	}

	return builder.String()
}

func (u *rustSubAction) codefileModBlock(codefile []*model.DeclareCodeFile, tabs string) string {
	var builder strings.Builder
	for _, c := range codefile {
		builder.WriteString(fmt.Sprintf("%spub mod %s;\n", tabs, c.Name))
	}

	return builder.String()
}