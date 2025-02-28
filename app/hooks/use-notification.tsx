import { X } from "lucide-react";
import { useEffect } from "react";
import { toast } from "sonner";

export function useNotification(message: string) {
  useEffect(() => {
    if (message !== "") {
      toast("Server response", {
        description: message,
        action: {
          label: <X size={16} />,
          onClick: () => {},
        },
      });
    }
  }, [message]);
}
