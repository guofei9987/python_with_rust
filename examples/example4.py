from python_with_rust import python_with_rust



# 创建 Recorder 的实例
recorder = python_with_rust.MyRecorder()

# 调用 func1 几次来增加计数
recorder.func1()
recorder.func1()
recorder.func1()

# 调用 func2 获取当前的计数
print(recorder.func2())  # 应该输出 3


