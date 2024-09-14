# Builder Pattern

`Builder`模式
* 需要一个生成中间构建器的函数（`builder()`）
* 在构建器上设置值的一系列方法
  * 每个set方法第一个参数必须是可变引用（`&mut self`）
  * 每个set方法的返回值也必须是自身的可变引用（`&mut Self`）以便它进行后续链式操作
* 从构建器构建最终值的方（`.build()`）
