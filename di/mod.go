package di

type DiContainer interface {}

type diContainer struct {}

func NewDiContainer() DiContainer {
	return &diContainer{}
}