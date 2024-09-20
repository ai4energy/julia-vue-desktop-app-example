using Oxygen
using HTTP
using JSON3

# 定义主要的路由函数，返回帮助信息
function main_help(req::HTTP.Request)
    return "using /ping to check the health of the API."
end

# 定义健康检查路由函数
function health_check(req::HTTP.Request)
    return Dict("ai4ejuliaapi" => "healthy!")
end

# 定义 `add` 函数来处理 `/add/{x}/{y}` 路由
function add(req::HTTP.Request, x, y)
    x_parsed = parse(Float64, x)
    y_parsed = parse(Float64, y)
    return string(x_parsed + y_parsed)  # 返回计算结果作为字符串
end

# 定义一个处理 POST 请求的函数
function add_post(req::HTTP.Request)
    # 解析 JSON 请求体
    data = JSON3.read(String(req.body))
    x = parse(Float64, data["x"])
    y = parse(Float64, data["y"])
    return string(x + y)  # 返回计算结果作为字符串
end

# 初始化主路由
function InitMainRouter()
    Oxygen.route([Oxygen.GET], "/", main_help)
    Oxygen.route([Oxygen.GET], "/ping", health_check)
    Oxygen.route([Oxygen.GET], "/add/{x}/{y}", add)
    Oxygen.route([Oxygen.POST], "/add_post", add_post)
end

# 初始化路由
function InitRouter()
    println("Initializing main router")
    InitMainRouter()
    # Ai4ECoolProp.InitRouter()
end

# 主函数，启动服务器
function julia_main()::Cint
    InitRouter()
    Oxygen.serve(host="0.0.0.0", port=19801, show_banner=false)
    return 0
end

# 启动服务器
julia_main()

# 测试
# 
# 启动服务
# julia --project=. .\julia-server.jl
# 
# 请求（如果是bash）
# curl -X POST http://localhost:19801/add_post -H "Content-Type: application/json" -d '{"x": "3.5", "y": "4.5"}'
