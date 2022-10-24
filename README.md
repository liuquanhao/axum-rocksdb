# axum + rocksdb

## 接口


### 创建todo

POST: /todos/

header: content-type:application/json

{"text": "todo1"}

返回：

header: Location:/todos/:id

### 获取某个todo

GET: /todos/:id

### 修改todo

PUT: /todos/:id

header: content-type:application/json

### 删除todo

DELETE: /todos/:id