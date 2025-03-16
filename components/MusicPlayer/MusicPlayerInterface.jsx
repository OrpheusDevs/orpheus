"use client";

import React, { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";
import { IAudioMetadata } from "music-metadata";
import * as musicMetadata from "music-metadata";
import Image from "next/image";

// SVG Icons as Components
const PlayIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="24"
    height="24"
    fill="currentColor"
    className="text-white"
  >
    <path d="M8 5v14l11-7z" />
  </svg>
);

const PauseIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="24"
    height="24"
    fill="currentColor"
    className="text-white"
  >
    <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
  </svg>
);

const ForwardIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="32"
    height="32"
    fill="currentColor"
    className="text-gray-300"
  >
    <path d="M4 18l8.5-6L4 6v12zm9-12v12l8.5-6L13 6z" />
  </svg>
);

const BackwardIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="32"
    height="32"
    fill="currentColor"
    className="text-gray-300"
  >
    <path d="M11 18V6l-8.5 6 8.5 6zm.5-6l8.5 6V6l-8.5 6z" />
  </svg>
);

const ShuffleIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="20"
    height="20"
    fill="currentColor"
    className="text-gray-300"
  >
    <path d="M10.59 9.17L5.41 4 4 5.41l5.17 5.17 1.42-1.41zM14.5 4l2.04 2.04L4 18.59 5.41 20 17.96 7.46 20 9.5V4h-5.5zm0.33 9.41l-1.41 1.41 3.13 3.13L14.5 20H20v-5.5l-2.04 2.04-3.13-3.13z" />
  </svg>
);

const RepeatIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="20"
    height="20"
    fill="currentColor"
    className="text-gray-300"
  >
    <path d="M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4z" />
  </svg>
);

const VolumeIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    width="20"
    height="20"
    fill="currentColor"
    className="text-gray-300"
  >
    <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z" />
  </svg>
);

// Utility function to convert Uint8Array to Base64
function uint8ArrayToBase64(buffer) {
  let binary = "";
  const bytes = new Uint8Array(buffer);
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

// Time formatting utility
const formatTime = (timeInSeconds) => {
  const minutes = Math.floor(timeInSeconds / 60);
  const seconds = Math.floor(timeInSeconds % 60);
  return `${minutes.toString().padStart(2, "0")}:${seconds
    .toString()
    .padStart(2, "0")}`;
};

const MusicPlayer = ({ audioSrc, count }) => {
  const [metadata, setMetadata] = useState(null);
  const [trackProgress, setTrackProgress] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(80);
  const [isShuffleOn, setIsShuffleOn] = useState(false);
  const [isRepeatOn, setIsRepeatOn] = useState(false);

  // Refs
  const audioRef = useRef(null);
  const intervalRef = useRef();

  // Initialize audio on client side
  useEffect(() => {
    if (typeof window !== "undefined") {
      audioRef.current = new Audio(audioSrc);
    }
  }, [audioSrc]);

  // Fetch and parse metadata
  useEffect(() => {
    const fetchAndParseMetadata = async () => {
      try {
        const response = await fetch(audioSrc);
        const blob = await response.blob();

        console.log(blob);

        const parsedMetadata = await musicMetadata.parseBlob(blob);
        setMetadata(parsedMetadata);
      } catch (error) {
        console.error(
          "Error fetching, converting, or parsing metadata:",
          error
        );
      }
    };

    if (typeof window !== "undefined") {
      fetchAndParseMetadata();
    }
  }, [audioSrc]);

  // Audio playback effect
  useEffect(() => {
    if (typeof window === "undefined") return;

    const audio = audioRef.current;
    if (!audio) return;

    const setAudioData = () => {
      setDuration(audio.duration);
    };

    const startTimer = () => {
      clearInterval(intervalRef.current);
      intervalRef.current = setInterval(() => {
        setTrackProgress(audio.currentTime);
      }, 1000);
    };

    audio.addEventListener("loadedmetadata", setAudioData);

    if (isPlaying) {
      audio.play().catch((e) => console.error("Error playing audio:", e));
      startTimer();
    } else {
      audio.pause();
      clearInterval(intervalRef.current);
    }

    return () => {
      audio.removeEventListener("loadedmetadata", setAudioData);
      clearInterval(intervalRef.current);
    };
  }, [isPlaying]);

  // Render album art
  const renderImage = () => {
    if (
      metadata &&
      metadata.common.picture &&
      metadata.common.picture.length > 0
    ) {
      const picture = metadata.common.picture[0];
      const base64String = uint8ArrayToBase64(picture.data);

      return base64String ? (
        <Image
          src={`data:${picture.format};base64,${base64String}`}
          width={200}
          height={200}
          className="h-full w-full object-cover"
          alt="Album Artwork"
        />
      ) : (
        <img
          src="/slowmotion.jpg"
          alt="Default Artwork"
          className="h-full w-full object-cover"
        />
      );
    }
    return (
      <img
        src="/slowmotion.jpg"
        alt="Default Artwork"
        className="h-full w-full object-cover"
      />
    );
  };

  // Seek functionality
  const handleSeek = (e) => {
    if (!audioRef.current) return;
    const seekTime = (e.nativeEvent.offsetX / e.target.offsetWidth) * duration;
    audioRef.current.currentTime = seekTime;
    setTrackProgress(seekTime);
  };

  // Skip functionality
  const handleSkip = (direction) => {
    if (!audioRef.current) return;
    const currentTime = audioRef.current.currentTime;
    const skipAmount = direction === "forward" ? 10 : -10;
    audioRef.current.currentTime = Math.max(
      0,
      Math.min(currentTime + skipAmount, duration)
    );
    setTrackProgress(audioRef.current.currentTime);
  };

  // Calculate progress percentage
  const currentPercentage = duration
    ? `${(trackProgress / duration) * 100}%`
    : "0%";

  return (
    <div className="bg-[#0a0a2a] text-white min-h-screen flex flex-col">
      {/* Main Interface */}
      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar - Navigation */}
        <div className="w-64 bg-[#0a0a2a] border-r border-[#1a1a4a] p-6 hidden md:block">
          <div className="mb-8">
            <h1 className="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-400 to-blue-500">
              ORPHEUS
            </h1>
          </div>

          <nav className="space-y-6">
            <div>
              <h3 className="text-xs uppercase text-gray-400 font-semibold mb-2">
                Library
              </h3>
              <ul className="space-y-2">
                <li className="group cursor-pointer hover:text-purple-400 transition">
                  <a className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
                      />
                    </svg>
                    My Music NFTs
                  </a>
                </li>
                <li className="group cursor-pointer hover:text-purple-400 transition">
                  <a className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                      />
                    </svg>
                    Liked Songs
                  </a>
                </li>
                <li className="group cursor-pointer hover:text-purple-400 transition">
                  <a className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                      />
                    </svg>
                    Playlists
                  </a>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-xs uppercase text-gray-400 font-semibold mb-2">
                Discover
              </h3>
              <ul className="space-y-2">
                <li className="group cursor-pointer hover:text-purple-400 transition">
                  <a className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
                      />
                    </svg>
                    NFT Marketplace
                  </a>
                </li>
                <li className="group cursor-pointer hover:text-purple-400 transition">
                  <a className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M13 10V3L4 14h7v7l9-11h-7z"
                      />
                    </svg>
                    New Releases
                  </a>
                </li>
                <li className="group cursor-pointer hover:text-purple-400 transition">
                  <a className="flex items-center">
                    <svg
                      className="w-5 h-5 mr-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                      />
                    </svg>
                    Top Artists
                  </a>
                </li>
              </ul>
            </div>
          </nav>
        </div>

        {/* Main Content Area */}
        <div className="flex-1 flex flex-col bg-gradient-to-b from-[#1a1a4a] to-[#0a0a2a] overflow-y-auto">
          {/* Current Album/Artist Header */}
          <div className="flex p-8 items-center">
            <div className="h-48 w-48 mr-6 shadow-lg rounded-md overflow-hidden">
              {renderImage()}
            </div>
            <div>
              <h2 className="text-xs uppercase text-gray-400">Song</h2>
              <h1 className="text-4xl font-bold mb-2">
                {metadata?.common?.title || "Unknown Title"}
              </h1>
              <div className="flex items-center">
                <span className="font-medium">
                  {metadata?.common?.artist || "Unknown Artist"}
                </span>
                <span className="mx-2 text-gray-400">•</span>
                <span className="text-gray-400">
                  {metadata?.common?.album || "Unknown Album"}
                </span>
              </div>
            </div>
          </div>

          {/* Player Controls */}
          <div className="px-8 pb-24">
            <div className="flex items-center mb-6">
              <motion.button
                className="flex items-center justify-center w-12 h-12 rounded-full bg-purple-600 mr-4"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                onClick={() => setIsPlaying(!isPlaying)}
              >
                {isPlaying ? <PauseIcon /> : <PlayIcon />}
              </motion.button>
              <div className="ml-4">
               
              </div>
            </div>

            {/* Playback Progress */}
            <div className="w-full flex items-center gap-2 mb-8">
              <span className="text-sm text-gray-400">
                {formatTime(trackProgress)}
              </span>
              <motion.div
                className="flex-grow h-2 bg-gray-700 rounded-full cursor-pointer"
                onClick={handleSeek}
              >
                <motion.div
                  className="h-full bg-purple-500 rounded-full"
                  style={{ width: currentPercentage }}
                  initial={{ scaleX: 0 }}
                  animate={{ scaleX: 1 }}
                  transition={{ duration: 0.2 }}
                />
              </motion.div>
              <span className="text-sm text-gray-400">
                {formatTime(duration)}
              </span>
            </div>

            {/* Additional Controls */}
            <div className="audio-controls w-full flex justify-center gap-6 mt-8">
              <motion.button
                type="button"
                className="prev p-2"
                aria-label="Previous"
                onClick={() => handleSkip("backward")}
                whileHover={{ scale: 1.1 }}
                whileTap={{ scale: 0.9 }}
              >
                <BackwardIcon />
              </motion.button>

              <motion.button
                type="button"
                className="next p-2"
                aria-label="Next"
                onClick={() => handleSkip("forward")}
                whileHover={{ scale: 1.1 }}
                whileTap={{ scale: 0.9 }}
              >
                <ForwardIcon />
              </motion.button>
            </div>

            {/* Metadata Display */}
            {metadata && (
              <div className="bg-[#1a1a4a] rounded-lg p-6 mt-8">
                <h3 className="text-lg font-semibold mb-4">
                  Track Information
                </h3>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-sm text-gray-400">Title</p>
                    <p>{metadata?.common?.title || "Unknown"}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-400">Artist</p>
                    <p>{metadata?.common?.artist || "Unknown"}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-400">Album</p>
                    <p>{metadata?.common?.album || "Unknown"}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-400">Year</p>
                    <p>{metadata?.common?.year || "Unknown"}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-400">Genre</p>
                    <p>{metadata?.common?.genre?.join(", ") || "Unknown"}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-400">Track</p>
                    <p>
                      {metadata?.common?.track?.no || "Unknown"} of{" "}
                      {metadata?.common?.track?.of || "?"}
                    </p>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Now Playing Bar */}
      <div className="h-20 bg-[#1a1a4a] border-t border-[#2a2a5a] px-4 flex items-center sticky bottom-0 z-10">
        <div className="w-1/3 flex items-center">
          <div className="h-14 w-14 mr-3 rounded overflow-hidden">
            {renderImage()}
          </div>
          <div>
            <div className="text-sm font-medium">
              {metadata?.common?.title || "Unknown Title"}
            </div>
            <div className="text-xs text-gray-400">
              {metadata?.common?.artist || "Unknown Artist"}
            </div>
          </div>
        </div>

        <div className="w-1/3 flex flex-col items-center">
          <div className="flex items-center mb-2">
            <button
              className={`mx-2 ${
                isShuffleOn
                  ? "text-purple-400"
                  : "text-gray-400 hover:text-white"
              }`}
              onClick={() => setIsShuffleOn(!isShuffleOn)}
            >
              <ShuffleIcon />
            </button>
            <button
              className="mx-2 text-gray-400 hover:text-white"
              onClick={() => handleSkip("backward")}
            >
              <BackwardIcon />
            </button>
            <motion.button
              className="flex items-center justify-center w-8 h-8 rounded-full bg-white mx-2"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              onClick={() => setIsPlaying(!isPlaying)}
            >
              {isPlaying ? (
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  width="16"
                  height="16"
                  fill="black"
                >
                  <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
                </svg>
              ) : (
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                  width="16"
                  height="16"
                  fill="black"
                >
                  <path d="M8 5v14l11-7z" />
                </svg>
              )}
            </motion.button>
            <button
              className="mx-2 text-gray-400 hover:text-white"
              onClick={() => handleSkip("forward")}
            >
              <ForwardIcon />
            </button>
            <button
              className={`mx-2 ${
                isRepeatOn
                  ? "text-purple-400"
                  : "text-gray-400 hover:text-white"
              }`}
              onClick={() => setIsRepeatOn(!isRepeatOn)}
            >
              <RepeatIcon />
            </button>
          </div>
          <div className="w-full flex items-center gap-2">
            <span className="text-xs text-gray-400">
              {formatTime(trackProgress)}
            </span>
            <div
              className="flex-grow h-1 bg-gray-700 rounded-full cursor-pointer"
              onClick={handleSeek}
            >
              <div
                className="h-full bg-purple-500 rounded-full relative"
                style={{ width: currentPercentage }}
              >
                <div className="absolute right-0 top-1/2 transform -translate-y-1/2 w-3 h-3 bg-white rounded-full opacity-0 hover:opacity-100"></div>
              </div>
            </div>
            <span className="text-xs text-gray-400">
              {formatTime(duration)}
            </span>
          </div>
        </div>

        <div className="w-1/3 flex justify-end items-center">
          <VolumeIcon />
          <div className="w-24 h-1 bg-gray-700 rounded-full mx-2 cursor-pointer">
            <div
              className="h-full bg-gray-400 rounded-full"
              style={{ width: `${volume}%` }}
            ></div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MusicPlayer;
