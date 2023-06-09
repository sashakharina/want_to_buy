import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
dayjs.extend(relativeTime)

export class DateUtil {
  static get ISOFormat() {
    return 'YYYY-MM-DDT00:00:00+00:00'
  }

  static toISO(date: dayjs.ConfigType, format?: dayjs.OptionType) {
    return this._instance(date, format).format(this.ISOFormat)
  }

  static toTimestamp(date?: dayjs.ConfigType, format?: dayjs.OptionType) {
    return this._instance(date, format).unix()
  }

  static fromTimestampSec(timestamp: number) {
    return dayjs.unix(timestamp)
  }

  static _instance(date?: dayjs.ConfigType, format?: dayjs.OptionType) {
    return format ? dayjs(date, format) : dayjs(date)
  }

  static timeLeft(date: dayjs.ConfigType) {
    if (!date) return 0
    const currentDate = this._instance()
    const targetDate = this._instance(date)

    return targetDate.diff(currentDate, 'ms')
  }

  static diffs(
    checkingDate: dayjs.ConfigType,
    targetDate: dayjs.ConfigType,
  ): number {
    return this._instance(checkingDate).diff(this._instance(targetDate))
  }

  static format(date: dayjs.ConfigType, format: string) {
    return this._instance(date).format(format)
  }

  static formatFromNow(date: dayjs.ConfigType) {
    return dayjs(date).fromNow()
  }
}
