import io.netty.buffer.ByteBuf;
import io.netty.buffer.Unpooled;
import io.netty.channel.ChannelFutureListener;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.ChannelInboundHandlerAdapter;
import io.netty.handler.codec.http.*;
import io.netty.util.CharsetUtil;

import java.lang.reflect.Method;
import java.net.InetAddress;

public class HttpServerHandler extends ChannelInboundHandlerAdapter {
    private String resutl = "";

    @Override
    public void channelRead(ChannelHandlerContext channelHandlerContext, Object msg) throws Exception {
        if (!(msg instanceof FullHttpRequest)) {
            resutl = "未知请求";
            send(channelHandlerContext, resutl, HttpResponseStatus.BAD_REQUEST);
            return;

        }
        FullHttpRequest httpRequest = (FullHttpRequest) msg;
        try {
            String path = httpRequest.uri();
            String body = getBody(httpRequest);
            HttpMethod httpMethod = httpRequest.method();
            if (!"/test".equalsIgnoreCase(path)) {
                resutl = "未定义请求";
                send(channelHandlerContext, resutl, HttpResponseStatus.NOT_FOUND);
                return;
            }
            System.out.println("接收到：" + httpMethod + "请求");
            if (HttpMethod.GET.equals(httpMethod)) {
                System.out.println("body" + body);
                resutl = "GET 请求";
                send(channelHandlerContext, resutl, HttpResponseStatus.OK);
                return;
            }
            if (HttpMethod.POST.equals(httpMethod)) {
                System.out.println("body" + body);
                resutl = "POST 请求";
                send(channelHandlerContext, resutl, HttpResponseStatus.OK);
                return;
            }

        } catch (Exception e) {
            System.out.println("请求处理失败");
            e.fillInStackTrace();
        } finally {
            httpRequest.release();
        }
    }

    @Override
    public void channelActive(ChannelHandlerContext ctx) throws Exception {
        System.out.println("连接的客户端地址:" + ctx.channel().remoteAddress());
        ctx.writeAndFlush("客户端" + InetAddress.getLocalHost().getHostName() + "成功与服务端建立连接！ ");
        super.channelActive(ctx);
    }


    private void send(ChannelHandlerContext ctx, String context, HttpResponseStatus status) throws Exception {
        FullHttpResponse response = new DefaultFullHttpResponse(HttpVersion.HTTP_1_1, status,
                Unpooled.copiedBuffer(context,CharsetUtil.UTF_8));
//        FullHttpResponse response = new DefaultFullHttpResponse(HttpVersion.HTTP_1_1, status);

        response.headers().set(HttpHeaderNames.CONTENT_TYPE, "text/plain");
        response.headers().set(HttpHeaderNames.CONTENT_LENGTH,
                response.content().readableBytes());


//        System.out.println(response.copy().content());
//        ctx.write(response);
        ctx.writeAndFlush(response).addListener(ChannelFutureListener.CLOSE);
//        ctx.flush();
//        ctx.close().sync();
    }

    @Override
    public void channelReadComplete(ChannelHandlerContext ctx) throws Exception {
        ctx.flush();
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        System.out.println(cause.getMessage());
        ctx.close();
    }

    private String getBody(FullHttpRequest request) {
        ByteBuf buf = request.content();
        return buf.toString(CharsetUtil.UTF_8);
    }


}
