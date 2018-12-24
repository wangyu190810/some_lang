import io.netty.channel.Channel;
import io.netty.channel.ChannelInitializer;
import io.netty.channel.ChannelPipeline;
import io.netty.channel.socket.SocketChannel;
import io.netty.handler.codec.http.HttpObjectAggregator;
import io.netty.handler.codec.http.HttpRequestDecoder;
import io.netty.handler.codec.http.HttpRequestEncoder;

public class HttpServerFilter extends ChannelInitializer<SocketChannel> {

    @Override
    protected void initChannel(SocketChannel channel) throws Exception{
        ChannelPipeline pipeline = channel.pipeline();
        pipeline.addLast("encoder",new HttpRequestEncoder());
        pipeline.addLast("decoder",new HttpRequestDecoder());
        pipeline.addLast("aggregator",new HttpObjectAggregator(10 * 1024 * 1024));
        pipeline.addLast("handler",new HttpServerHandler());
    }
}
