import React, { useState } from "react";
import { useTheme } from "../utils/theme-provider";
import formatTimestamp from "@/utils/format-timestamp";
import { ChatEntry } from "@/app/page";

const ChatMessage = ({ source, timestamp, content }: ChatEntry) => {
  const { theme } = useTheme();
  const inputColor = theme.overlay;
  const textPrimary = theme.textPrimary;
  const textSecondary = theme.textSecondary;

  let [selected, setSelected] = useState(false);

  // Determine the text color based on the selected state
  const textColor = selected ? textPrimary : textSecondary;

  return (
    <div>
      <div className="flex items-start gap-2.5">
        <div className="flex flex-col gap-1 w-full max-w-[320px]">
          <div
            style={{ backgroundColor: inputColor }}
            className="flex flex-col leading-1.5 p-4 opacity-60 rounded-e-xl rounded-es-xl"
          >
            <p style={{ color: textColor, fontSize: "8px" }}>
              {formatTimestamp(timestamp)}
            </p>
            <p
              onMouseEnter={() => setSelected(true)}
              onMouseLeave={() => setSelected(false)}
              className="text-sm font-normal cursor-pointer"
              style={{ color: textColor }}
            >
              {content}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ChatMessage;
