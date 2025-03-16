import MusicPlayer from "@/components/MusicPlayer/MusicPlayerInterface";
import React from "react";

const page = () => {
  return (
    <div>
      <MusicPlayer audioSrc="https://orpheus2.s3.ap-south-1.amazonaws.com/songs/Sign.mp3" count="42" />
    </div>
  );
};

export default page;
