import {Button} from "@/components/ui/button.tsx";
import {Link} from "react-router-dom";

function App() {

  return (
    <main className="container">
      <h1 className={"text-3xl"}>Welcome to Tauri + React</h1>
        <Link to="/login">
            <Button>Click</Button>
        </Link>
    </main>
  );
}

export default App;
