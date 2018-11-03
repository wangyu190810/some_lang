package Hello;

import io.netty.buffer.ByteBuf;
//import io.netty.buffer.ChannelBuf;
import io.netty.buffer.Unpooled;
import io.netty.channel.*;
import io.netty.util.CharsetUtil;
//import org.jboss.netty.channel.ChannelHandler.Sharable;

@ChannelHandler.Sharable
public class EchoServerHandler extends ChannelInboundHandlerAdapter{


    @Override
    public void channelRead(ChannelHandlerContext ctx, Object msg){
        ByteBuf in  = (ByteBuf) msg;
        System.out.print(
                "Server received :" + in.toString(CharsetUtil.UTF_8)

        );
    }
    @Override
    public void channelReadComplete(ChannelHandlerContext cxt){
        cxt.write(Unpooled.BIG_ENDIAN).addListener(ChannelFutureListener.CLOSE);
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
                 // Close the connection when an exception is raised.
                  cause.printStackTrace();
                  ctx.close();
              }
}
