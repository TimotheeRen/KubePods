import { Wheat } from "lucide-react";
import logo from "@/assets/logo.png"

const Footer = () => {
  return (
    <footer className="flex items-center justify-between border-t bg-background px-6 py-4">
      <a href="#" className="flex gap-1">
        <img src={logo} alt="Logo" className="w-7" />
        <span className="text-lg">KubePods</span>
      </a>

      <p className="font-medium text-muted-foreground text-sm">
        Copyright &copy; {new Date().getFullYear()} KubePods. All rights
        reserved.
      </p>
    </footer>
  );
};

export default Footer;
