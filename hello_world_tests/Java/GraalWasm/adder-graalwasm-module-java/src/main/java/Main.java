import java.io.IOException;
import java.net.URL;

import org.graalvm.polyglot.Context;
import org.graalvm.polyglot.Source;
import org.graalvm.polyglot.Value;



public class Main {
    public static void main(String[] args) {
        try (Context context = Context.create()) {
            URL wasmFile = Main.class.getResource("add-two.wasm");
            Value mainModule = context.eval(Source.newBuilder("wasm", wasmFile).build());
            Value mainInstance = mainModule.newInstance();
            Value addTwo = mainInstance.getMember("exports").getMember("addTwo");
            System.out.println("addTwo(40, 2) = " + addTwo.execute(40, 2));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}