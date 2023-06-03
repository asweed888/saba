package usecase

import (
	"io/ioutil"
	"os"
	"strconv"

	"github.com/asweed888/saba/domain/repository"
	"github.com/asweed888/saba/utils"
)

type InitialDeclareUseCase interface {
	GenerateInitialDeclareFile(lang string, isDDD string) error
}

type initialDeclareUseCase struct {
    repository.InitialDeclareRepository
}

func NewInitialDeclareUseCase(r repository.InitialDeclareRepository) InitialDeclareUseCase {
    return &initialDeclareUseCase{r}
}


func (u *initialDeclareUseCase) GenerateInitialDeclareFile(lang string, isDDD string) error {

	idec, err := u.GetInitialDeclareFileContents(lang, isDDD)
	if err != nil {
		return utils.Error(err)
	}

	perm32, _ := strconv.ParseUint("0644", 8, 32)

	if _, err := os.Stat("./saba.yml"); err != nil {
		err = ioutil.WriteFile("./saba.yml", []byte(idec.FileContents), os.FileMode(perm32))
		if err != nil { return utils.Error(err) }
	}

	return nil
}