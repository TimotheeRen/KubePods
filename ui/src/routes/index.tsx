import LoginDialog from "@/components/loginDialog";
import RegisterDialog from "@/components/registerDialog";

export default function Root() {
  return (
    <div>
      <h1>KubePods</h1>
      <LoginDialog />
      <RegisterDialog />
    </div>
  )
}
